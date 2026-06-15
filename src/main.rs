use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use open_social_data_core::{
    CachedDataset, FetchOptions, FetchResult, LocalCatalog, ProviderRegistry, QualityStatus,
    provider_payload_assertions, sync_catalog_path_from_registry, validate_quality,
    write_parquet_atomic,
};

#[derive(Debug, Parser)]
#[command(name = "open-social-data-cli")]
#[command(about = "Fetch and inspect open social datasets")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    List {
        #[arg(short, long)]
        provider: Option<String>,
    },
    Catalog {
        #[command(subcommand)]
        command: CatalogCommand,
    },
    Status,
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
}

#[derive(Debug, Subcommand)]
enum CatalogCommand {
    List {
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(short, long)]
        provider: Option<String>,
    },
    Search {
        query: String,
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(short, long)]
        provider: Option<String>,
    },
    Sync {
        #[arg(short, long, default_value = ".open-social-data/catalog.json")]
        path: PathBuf,
        #[arg(short, long)]
        provider: Option<String>,
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
        Commands::Catalog { command } => run_catalog_command(command, &registry).await?,
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

async fn run_catalog_command(
    command: CatalogCommand,
    registry: &ProviderRegistry,
) -> anyhow::Result<()> {
    match command {
        CatalogCommand::List { path, provider } => {
            let catalog = LocalCatalog::load(&path)?;
            print_catalog_rows(catalog.list(provider.as_deref()));
        }
        CatalogCommand::Search {
            query,
            path,
            provider,
        } => {
            let catalog = LocalCatalog::load(&path)?;
            print_catalog_rows(catalog.search(&query, provider.as_deref()));
        }
        CatalogCommand::Sync { path, provider } => {
            let report = sync_catalog_path_from_registry(
                &path,
                registry,
                provider.as_deref(),
                unix_timestamp_string(),
            )
            .await;
            for provider_name in &report.synced_providers {
                println!("{} synced {}", "ok".green(), provider_name);
            }
            println!(
                "synced {} dataset metadata record(s)",
                report.synced_records
            );
            if report.partial_success {
                println!("partial sync results were saved to {}", path.display());
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

fn print_catalog_rows(rows: Vec<&CachedDataset>) {
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
