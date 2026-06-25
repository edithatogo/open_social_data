use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use clap::{CommandFactory, Parser, Subcommand};
use clap::error::ErrorKind;
use clap_markdown;
use clap_complete::{Generator, Shell};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use open_social_data_core::{
    CachedDataset, FetchOptions, FetchResult, LocalCatalog, ProviderRegistry, QualityStatus,
    SqliteCatalog, provider_payload_assertions, read_parquet, sync_catalog_path_from_registry,
    sync_sqlite_catalog_path_from_registry, validate_quality, write_parquet_atomic,
};
use serde_json::Value;

#[derive(Debug, Parser)]
#[command(name = "open-social-data-cli")]
#[command(about = "Fetch and inspect open social datasets")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List available providers and their datasets
    List {
        #[arg(short, long)]
        provider: Option<String>,
    },
    /// Manage the local dataset catalog
    Catalog {
        #[command(subcommand)]
        command: CatalogCommand,
    },
    /// Validate dataset packs, metadata, and examples
    Validate {
        #[command(subcommand)]
        command: ValidateCommand,
    },
    /// Run example commands against local data
    Examples {
        #[command(subcommand)]
        command: ExampleCommand,
    },
    /// Show CLI and catalog status
    Status,
    /// Fetch a dataset from a provider
    Fetch {
        provider: String,
        dataset_id: String,
        #[arg(short, long)]
        output: PathBuf,
        #[arg(long, default_value = ".open-social-data/catalog.json")]
        catalog: PathBuf,
        #[arg(long)]
        quality_report: Option<PathBuf>,
    },
    /// Generate shell completions or man pages
    Generate {
        #[command(subcommand)]
        command: GenerateCommand,
    },
}

#[derive(Debug, Subcommand)]
enum ValidateCommand {
    DatasetPacks {
        #[arg(long, default_value = "datasets")]
        root: PathBuf,
    },
    SourceMetadata {
        #[arg(long, default_value = "datasets")]
        root: PathBuf,
        #[arg(long)]
        require_all: bool,
    },
    MediumTerm {
        #[arg(long)]
        run_examples: bool,
    },
}

#[derive(Debug, Subcommand)]
enum ExampleCommand {
    MyhospitalsSummary {
        #[arg(long, default_value = "datasets/aihw/myhospitals/data")]
        data_dir: PathBuf,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    SourceMetadataInventory {
        #[arg(long, default_value = "datasets")]
        root: PathBuf,
    },
}

#[derive(Debug, Subcommand)]
enum CatalogCommand {
    List {
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(long)]
        sqlite: Option<PathBuf>,
        #[arg(short, long)]
        provider: Option<String>,
    },
    Search {
        query: String,
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(long)]
        sqlite: Option<PathBuf>,
        #[arg(short, long)]
        provider: Option<String>,
    },
    Sync {
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(long)]
        sqlite: Option<PathBuf>,
        #[arg(short, long)]
        provider: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
enum GenerateCommand {
    /// Generate shell completion scripts
    Completions {
        /// Shell type (bash, zsh, fish, powershell, elvish)
        shell: Shell,
    },
    /// Generate CLI man page (markdown)
    ManPage {
        /// Output directory for the man page
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let registry = ProviderRegistry::with_defaults();

    match cli.command {
        Commands::List { provider } => {
            if let Some(provider_name) = provider {
                let provider = registry.get(&provider_name)?;
                let catalog = provider.list_datasets().await?;
                for dataset in catalog.datasets {
                    println!("{} {}", dataset.id.green(), dataset.name);
                }
            } else {
                for name in registry.names() {
                    println!("{name}");
                }
            }
        }
        Commands::Generate { command } => run_generate_command(command)?,
        Commands::Catalog { command } => run_catalog_command(command, &registry).await?,
        Commands::Validate { command } => run_validate_command(command)?,
        Commands::Examples { command } => run_example_command(command)?,
        Commands::Status => {
            for name in registry.names() {
                let provider = registry.get(name)?;
                let result = provider.ping().await;
                match result {
                    Ok(()) => println!("{} {}", "ok".green(), name),
                    Err(error) => println!("{} {}: {}", "error".red(), name, error),
                }
            }
        }
        Commands::Fetch {
            provider,
            dataset_id,
            output,
            catalog,
            quality_report,
        } => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(
                ProgressStyle::with_template("{spinner} {msg}")
                    .context("failed to configure progress spinner")?,
            );
            spinner.enable_steady_tick(std::time::Duration::from_millis(120));
            spinner.set_message(format!("fetching {dataset_id} from {provider}"));

            let provider_name = provider;
            let provider = registry.get(&provider_name)?;
            let mut local_catalog = LocalCatalog::load(&catalog)?;
            let conditional = local_catalog
                .get(&provider_name, &dataset_id)
                .filter(|dataset| {
                    dataset
                        .output_path
                        .as_ref()
                        .is_some_and(|path| path.exists())
                })
                .map(CachedDataset::conditional_request_metadata)
                .unwrap_or_default();
            let fetch_result = provider
                .fetch_dataset_with_options(&dataset_id, FetchOptions::new(conditional))
                .await?;
            let timestamp = unix_timestamp_string();
            let (frame, etag, last_modified) = match fetch_result {
                FetchResult::Fetched {
                    frame,
                    etag,
                    last_modified,
                } => (frame, etag, last_modified),
                FetchResult::NotModified {
                    etag,
                    last_modified,
                } => {
                    local_catalog.mark_not_modified(
                        provider_name,
                        dataset_id,
                        timestamp,
                        etag,
                        last_modified,
                    );
                    local_catalog.save_atomic(&catalog)?;
                    spinner.finish_with_message("not modified; existing output preserved");
                    return Ok(());
                }
            };
            let row_count = frame.height();
            let report = validate_quality(&frame, &provider_payload_assertions())?;
            let quality_status = if report.is_valid() {
                QualityStatus::Passed
            } else {
                QualityStatus::Failed
            };
            if let Some(path) = quality_report.as_ref() {
                report.save_atomic(path)?;
            }
            report.into_result()?;
            spinner.set_message("writing parquet");
            write_parquet_atomic(&frame, &output)?;
            local_catalog.upsert_fetch_result(CachedDataset {
                provider: provider_name,
                dataset_id,
                name: None,
                version: None,
                catalog_synced_at: None,
                last_fetched_at: Some(timestamp),
                last_not_modified_at: None,
                source_url: None,
                etag,
                last_modified,
                output_path: Some(output.clone()),
                quality_status: Some(quality_status),
                quality_report_path: quality_report,
                row_count: Some(row_count),
                not_modified_count: 0,
            });
            local_catalog.save_atomic(&catalog)?;
            spinner.finish_with_message(format!("wrote {}", output.display()));
        }
    }

    Ok(())
}

fn run_validate_command(command: ValidateCommand) -> anyhow::Result<()> {
    match command {
        ValidateCommand::DatasetPacks { root } => {
            validate_dataset_packs(&root)?;
        }
        ValidateCommand::SourceMetadata { root, require_all } => {
            validate_source_metadata(&root, require_all)?;
        }
        ValidateCommand::MediumTerm { run_examples } => {
            validate_medium_term(run_examples)?;
        }
    }
    Ok(())
}

fn run_example_command(command: ExampleCommand) -> anyhow::Result<()> {
    match command {
        ExampleCommand::MyhospitalsSummary { data_dir, limit } => {
            print_myhospitals_summary(&data_dir, limit)?;
        }
        ExampleCommand::SourceMetadataInventory { root } => {
            print_source_metadata_inventory(&root)?;
        }
    }
    Ok(())
}

const REQUIRED_PACK_SOURCES: &[&str] = &["abs", "stats_nz", "aihw", "moh"];

fn dataset_dirs(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    if !root.is_dir() {
        anyhow::bail!("dataset root does not exist: {}", root.display());
    }
    for source in fs::read_dir(root)? {
        let source = source?.path();
        if !source.is_dir() {
            continue;
        }
        for dataset in fs::read_dir(source)? {
            let dataset = dataset?.path();
            if dataset.is_dir()
                && dataset
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| !name.starts_with('.'))
            {
                dirs.push(dataset);
            }
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn dataset_pack_dirs(root: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();
    if !root.is_dir() {
        anyhow::bail!("dataset root does not exist: {}", root.display());
    }
    for source in REQUIRED_PACK_SOURCES {
        let source_dir = root.join(source);
        if !source_dir.exists() {
            continue;
        }
        for dataset in fs::read_dir(source_dir)? {
            let dataset = dataset?.path();
            if dataset.is_dir()
                && dataset
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| !name.starts_with('.'))
            {
                dirs.push(dataset);
            }
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn has_child_matching(dir: &Path, predicate: impl Fn(&Path) -> bool) -> bool {
    fs::read_dir(dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .any(|path| predicate(&path))
}

fn dataset_has_pack_shape(path: &Path) -> bool {
    let docs = path.join("docs");
    let scripts = path.join("scripts");
    path.join("README.md").is_file()
        && path.join("SESSION_LOG.md").is_file()
        && docs.is_dir()
        && has_child_matching(&docs, |path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("data_dictionary") && name.ends_with(".md"))
        })
        && has_child_matching(&docs, |path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("accessible_guide") && name.ends_with(".md"))
        })
        && scripts.is_dir()
        && has_child_matching(&scripts, |path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext == "py")
        })
}

fn validate_dataset_packs(root: &Path) -> anyhow::Result<usize> {
    let mut failed = false;
    let mut count = 0;
    for dataset in dataset_pack_dirs(root)? {
        count += 1;
        if dataset_has_pack_shape(&dataset) {
            println!("{} {}", "OK".green(), dataset.display());
        } else {
            failed = true;
            println!(
                "{} {}: missing README, SESSION_LOG, docs, or script",
                "FAIL".red(),
                dataset.display()
            );
        }
    }
    if failed {
        anyhow::bail!("dataset pack validation failed");
    }
    Ok(count)
}

fn validate_source_metadata(root: &Path, require_all: bool) -> anyhow::Result<usize> {
    let required = [
        "source_agency",
        "source_title",
        "source_url",
        "access_method",
        "licence",
        "update_cadence",
        "methodology_url",
        "units",
        "codelists",
        "caveats",
        "official_metadata_status",
    ];
    let mut failed = false;
    let mut checked = 0;
    for dataset in dataset_dirs(root)? {
        let path = dataset.join("source_metadata.json");
        if !path.is_file() && !require_all {
            continue;
        }
        checked += 1;
        if !path.is_file() {
            failed = true;
            println!(
                "{} {}: missing source_metadata.json",
                "FAIL".red(),
                dataset.display()
            );
            continue;
        }
        let value: Value = match serde_json::from_str(&fs::read_to_string(&path)?) {
            Ok(value) => value,
            Err(error) => {
                failed = true;
                println!(
                    "{} {}: invalid JSON: {}",
                    "FAIL".red(),
                    dataset.display(),
                    error
                );
                continue;
            }
        };
        let missing = required
            .iter()
            .filter(|field| value.get(**field).is_none())
            .copied()
            .collect::<Vec<_>>();
        let bad_lists = ["units", "codelists", "caveats"]
            .iter()
            .filter(|field| {
                value
                    .get(**field)
                    .is_some_and(|item| item.as_array().is_none_or(|items| items.is_empty()))
            })
            .copied()
            .collect::<Vec<_>>();
        if missing.is_empty() && bad_lists.is_empty() {
            println!("{} {}", "OK".green(), dataset.display());
        } else {
            failed = true;
            println!(
                "{} {}: missing fields [{}], empty list fields [{}]",
                "FAIL".red(),
                dataset.display(),
                missing.join(", "),
                bad_lists.join(", ")
            );
        }
    }
    if checked == 0 {
        anyhow::bail!("no source_metadata.json files found");
    }
    if failed {
        anyhow::bail!("source metadata validation failed");
    }
    Ok(checked)
}
fn validate_medium_term(run_examples: bool) -> anyhow::Result<()> {
    let datasets = Path::new("datasets");
    let packs = dataset_dirs(datasets)?
        .into_iter()
        .filter(|path| dataset_has_pack_shape(path))
        .collect::<Vec<_>>();
    let has_new_source = packs.iter().any(|path| {
        path.parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .is_some_and(|source| !matches!(source, "abs" | "stats_nz" | "aihw"))
    });
    let required_guides = [
        "docs/guides/understanding-social-statistics-concepts.md",
        "docs/guides/interpreting-common-visualizations.md",
        "docs/guides/ethical-use-of-social-data.md",
    ];
    let metadata_packs = dataset_dirs(datasets)?
        .into_iter()
        .filter(|path| path.join("source_metadata.json").is_file())
        .count();
    if !Path::new("conductor/tracks/medium_term_expansion_20260618/dataset_candidates.md").is_file()
    {
        anyhow::bail!("missing medium-term dataset candidate backlog");
    }
    if packs.len() < 11 {
        anyhow::bail!(
            "expected at least 11 shaped dataset packs, found {}",
            packs.len()
        );
    }
    if !has_new_source {
        anyhow::bail!("expected at least one non-ABS/Stats NZ/AIHW dataset pack");
    }
    for guide in required_guides {
        if !Path::new(guide).is_file() {
            anyhow::bail!("missing guide {guide}");
        }
    }
    if metadata_packs < 3 {
        anyhow::bail!(
            "expected at least three source metadata files, found {}",
            metadata_packs
        );
    }
    validate_source_metadata(datasets, false)?;
    if run_examples {
        print_myhospitals_summary(Path::new("datasets/aihw/myhospitals/data"), 5)?;
        print_source_metadata_inventory(datasets)?;
    }
    println!("{} medium-term roadmap artefacts", "OK".green());
    Ok(())
}

fn print_myhospitals_summary(data_dir: &Path, limit: usize) -> anyhow::Result<()> {
    let mut rows = Vec::new();
    for entry in
        fs::read_dir(data_dir).with_context(|| format!("reading {}", data_dir.display()))?
    {
        let path = entry?.path();
        if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_none_or(|ext| ext != "parquet")
        {
            continue;
        }
        let frame = read_parquet(&path)?;
        rows.push((
            path.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("unknown")
                .to_string(),
            frame.height(),
            frame.width(),
        ));
    }
    if rows.is_empty() {
        anyhow::bail!("no Parquet files found in {}", data_dir.display());
    }
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    println!("file\trows\tcolumns");
    for (file, row_count, column_count) in rows.into_iter().take(limit) {
        println!("{file}\t{row_count}\t{column_count}");
    }
    Ok(())
}

fn print_source_metadata_inventory(root: &Path) -> anyhow::Result<()> {
    println!("dataset\tsource_agency\tupdate_cadence\tunits\tcodelists");
    let mut printed = 0;
    for dataset in dataset_dirs(root)? {
        let path = dataset.join("source_metadata.json");
        if !path.is_file() {
            continue;
        }
        let value: Value = serde_json::from_str(&fs::read_to_string(&path)?)?;
        let units = value
            .get("units")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);
        let codelists = value
            .get("codelists")
            .and_then(Value::as_array)
            .map_or(0, Vec::len);
        println!(
            "{}\t{}\t{}\t{}\t{}",
            dataset.strip_prefix(root).unwrap_or(&dataset).display(),
            value
                .get("source_agency")
                .and_then(Value::as_str)
                .unwrap_or(""),
            value
                .get("update_cadence")
                .and_then(Value::as_str)
                .unwrap_or(""),
            units,
            codelists
        );
        printed += 1;
    }
    if printed == 0 {
        anyhow::bail!("no source_metadata.json files found");
    }
    Ok(())
}
async fn run_catalog_command(
    command: CatalogCommand,
    registry: &ProviderRegistry,
) -> anyhow::Result<()> {
    match command {
        CatalogCommand::List {
            path,
            sqlite,
            provider,
        } => {
            if let Some(sqlite_path) = sqlite {
                let catalog = SqliteCatalog::open(&sqlite_path)?;
                print_catalog_rows(catalog.list(provider.as_deref())?.iter());
            } else {
                let catalog = LocalCatalog::load(&path)?;
                print_catalog_rows(catalog.list(provider.as_deref()));
            }
        }
        CatalogCommand::Search {
            query,
            path,
            sqlite,
            provider,
        } => {
            if let Some(sqlite_path) = sqlite {
                let catalog = SqliteCatalog::open(&sqlite_path)?;
                print_catalog_rows(catalog.search(&query, provider.as_deref())?.iter());
            } else {
                let catalog = LocalCatalog::load(&path)?;
                print_catalog_rows(catalog.search(&query, provider.as_deref()));
            }
        }
        CatalogCommand::Sync {
            path,
            sqlite,
            provider,
        } => {
            let output_path = sqlite.as_ref().unwrap_or(&path);
            let report = if let Some(sqlite_path) = sqlite.as_ref() {
                sync_sqlite_catalog_path_from_registry(
                    sqlite_path,
                    registry,
                    provider.as_deref(),
                    unix_timestamp_string(),
                )
                .await?
            } else {
                sync_catalog_path_from_registry(
                    &path,
                    registry,
                    provider.as_deref(),
                    unix_timestamp_string(),
                )
                .await?
            };
            for provider_name in &report.synced_providers {
                println!("{} synced {}", "ok".green(), provider_name);
            }
            println!(
                "synced {} dataset metadata record(s)",
                report.synced_records
            );
            if report.partial_success {
                println!(
                    "partial sync results were saved to {}",
                    output_path.display()
                );
            }
            if report.has_errors() {
                for error in &report.errors {
                    println!("{} {}: {}", "error".red(), error.provider, error.message);
                }
                anyhow::bail!(
"catalog sync completed with errors: {}",
                    report.error_summary()
                );
            }
        }
        }
    Ok(())
}

fn run_generate_command(command: GenerateCommand) -> anyhow::Result<()> {
    match command {
        GenerateCommand::Completions { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            clap_complete::generate(shell, &mut cmd, bin_name, &mut std::io::stdout());
        }
        GenerateCommand::ManPage { output } => {
            let cmd = Cli::command();
            let man = clap_markdown::help_markdown(&cmd);
            let output_path = output.join(format!("{}.md", cmd.get_name()));
            fs::write(&output_path, man).with_context(|| {
                format!("failed to write man page to {}", output_path.display())
            })?;
            println!("{} man page written to {}", "ok".green(), output_path.display());
        }
    }
    Ok(())
}

fn print_catalog_rows<'a>(rows: impl IntoIterator<Item = &'a CachedDataset>) {
    let rows = rows.into_iter().collect::<Vec<_>>();
    if rows.is_empty() {
        println!("catalog is empty");
        return;
    }

    for dataset in rows {
        let row_count = dataset
            .row_count
            .map(|count| count.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        println!(
            "{} {} rows={} quality={} synced={} etag={} output={}",
            dataset.provider.green(),
            dataset.dataset_id,
            row_count,
            dataset
                .quality_status
                .map(|status| format!("{status:?}").to_ascii_lowercase())
                .unwrap_or_else(|| "unknown".to_string()),
            dataset.catalog_synced_at.as_deref().unwrap_or("-"),
            dataset.etag.as_deref().unwrap_or("-"),
            dataset
                .output_path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "-".to_string())
        );
    }
}

fn unix_timestamp_string() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string())
}
