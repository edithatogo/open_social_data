# Data.gov.au provider

## Goal

Expose the official Data.gov.au CKAN API through the shared provider registry,
starting with the OAIC aggregate freedom-of-information statistics dataset.

## Requirements

- Use documented public CKAN endpoints.
- Discover the current CSV resource from package metadata.
- Preserve source URL and conditional response metadata.
- Keep the provider separate from Right to Know request archival.
- Provide hermetic CSV parsing and registry tests.
