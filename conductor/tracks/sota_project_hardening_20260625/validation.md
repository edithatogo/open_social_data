# Validation Evidence

## 2026-06-25

Track created from the SOTA/bleeding-edge readiness review.

Planned validation commands:

```cmd
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo nextest run
cargo audit
cargo deny check
cargo llvm-cov --all-features --workspace
npm install
npm run build
git diff --check
```

Notes:

- Commands that require new tools should be added with installation guidance or CI setup during implementation.
- Release, upload, package-publish, or artifact-signing actions require explicit user approval and should not run as local validation by default.
