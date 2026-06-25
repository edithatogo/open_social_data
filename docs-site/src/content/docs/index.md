---
title: Open Social Data
description: Comprehensive documentation for the Open Social Data project.
template: splash
hero:
  title: Open Social Data
  tagline: Download, process, and export public social datasets from national statistics agencies into modern formats like Parquet.
  actions:
    - text: Get started
      link: /open_social_data/getting-started/
      icon: right-arrow
    - text: CLI reference
      link: /open_social_data/cli-reference/
      icon: open-book
---

import { Card, CardGrid } from '@astrojs/starlight/components';

This documentation portal covers the Open Social Data CLI tool, dataset pack structure, provider authoring, catalog usage, validation, and project roadmap.

<CardGrid>
  <Card title="Getting Started" icon="rocket">
    Install the Rust toolchain, build the CLI, and run your first commands.
  </Card>
  <Card title="CLI Reference" icon="open-book">
    Complete reference for `list`, `status`, `fetch`, `catalog`, and `validate` commands.
  </Card>
  <Card title="Providers" icon="setting">
    Learn about the DatasetProvider trait and how to add new data sources.
  </Card>
  <Card title="Dataset Packs" icon="document">
    Structure and create dataset packs using the standard templates.
  </Card>
  <Card title="Catalog" icon="database">
    Use the JSON or SQLite catalog to manage dataset metadata locally.
  </Card>
  <Card title="Validation" icon="check">
    Run validation gates, quality assertions, and maintenance checks.
  </Card>
</CardGrid>

## Quick links

- **[Project Roadmap](/open_social_data/roadmap-status/)** — current track status and completed tracks
- **[GitHub Repository](https://github.com/edithatogo/open_social_data)** — source code and issues
- **[Existing Guides](/docs/guides/)** — ethical use, statistics concepts, and visualization guides

