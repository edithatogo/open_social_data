# Open Social Datasets Initiative

**Our mission is to make open social data from sources like national statistics agencies (e.g., Stats NZ, ABS) accessible, understandable, and usable for a wide audience.**

This repository aims to be a central hub for curated links to datasets, their comprehensive documentation, and guides to help everyone from researchers to the general public explore and utilize valuable social information.

## Project Philosophy

*   **Openness & Accessibility:** Publicly available data and documentation under open licenses.
*   **Understandability:** Clear, concise documentation for technical and non-technical users, including plain language guides.
*   **Usability & Reusability:** Common data formats and clear access instructions.
*   **Transparency & Reproducibility:** Documented processes for data handling.
*   **Collaboration & Community:** Encouraging contributions and feedback.
*   **Accuracy & Reliability:** Striving for up-to-date information, clearly linking to original sources.
*   **Ethical Considerations:** Highlighting responsible data use.

## How to Navigate This Repository

*   **/datasets**: Contains organized collections of datasets from various sources.
    *   Each dataset will have its own directory including:
        *   `README.md`: Specific information about the dataset, its source, structure, and how to access it.
        *   `data_dictionary.md`: Detailed explanation of all variables.
        *   `accessible_guide.md`: A plain-language guide to understanding the dataset.
        *   Potentially `data/` (for small, directly hosted data snippets or examples) or `scripts/` (for fetching/processing).
*   **/docs**: General guides, technical documentation, and articles related to using social data.
    *   `docs/guides/`: Accessible, topic-based guides.
    *   `docs/technical/`: More in-depth technical papers or API usage examples.
*   **/templates**: Contains templates for creating standardized dataset documentation.
*   **ROADMAP.md**: Outlines future plans and milestones for this project.
*   **TODO.md**: Lists specific tasks to be done.
*   **CHANGELOG.md**: Tracks changes and additions to the repository.
*   **CONTRIBUTING.md**: Guidelines on how to contribute to this project (to be developed further).
*   **SESSION_LOG_TEMPLATE.md**: A template for documenting data work sessions.

## Rust Data Engine Preview

This repository now includes an early Rust data engine and CLI scaffold for provider-backed data fetches, quality checks, Parquet export, and local catalog metadata.

Common commands:

```bash
cargo run --bin open-social-data-cli -- list
cargo run --bin open-social-data-cli -- list --provider abs
cargo run --bin open-social-data-cli -- status
cargo run --bin open-social-data-cli -- fetch abs QBIS --output datasets/abs/qbis_business_indicators/data/qbis.parquet --quality-report .open-social-data/qbis-quality.json
cargo run --bin open-social-data-cli -- catalog sync
cargo run --bin open-social-data-cli -- catalog sync --provider abs
cargo run --bin open-social-data-cli -- catalog list
cargo run --bin open-social-data-cli -- catalog search qbis
```

Local runtime metadata is written under `.open-social-data/` by default. The current catalog is JSON-backed so it can work without native SQLite or DuckDB dependencies while the Windows MSVC/SDK linker setup is unresolved. Catalog entries preserve provider source URLs, quality report paths, ETag values, and Last-Modified values. Fetches reuse cached ETag and Last-Modified metadata and preserve existing outputs when a provider returns `304 Not Modified`.

## How to Contribute

We welcome contributions! Please see `CONTRIBUTING.md` for more details (once fully developed). In general, contributions can include:
*   Adding new datasets (following the established structure and templates).
*   Improving documentation.
*   Reporting issues or suggesting improvements.
*   Sharing use cases or analysis scripts.

## License

*   The content of this repository, including documentation and scripts generated for this project, is licensed under the [MIT License](LICENSE) unless otherwise stated.
*   Individual datasets will clearly state their original source licenses. Please ensure you adhere to those terms.

## Contact & Acknowledgements

*   For questions or suggestions regarding this project, please open an issue in the repository.
*   We acknowledge and thank the national statistics agencies and other open data providers for making their data publicly available.

---
*This README provides an overview. For more detailed plans, see ROADMAP.md.*
