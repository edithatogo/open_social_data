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

## 2026-08-10 completion evidence

- Local GNU Rust validation passed: `cargo fmt --check`, `cargo check --all-targets`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo test --all-targets`.
- Local test evidence: 54 library tests, 7 CLI integration tests, and 4 parser
  regression tests passed.
- Local documentation validation passed: `npm run check --prefix docs-site` and
  `npm run build --prefix docs-site` (12 static pages built).
- Hosted CI run `30754406767` on `main` passed the repository CI matrix,
  including nextest, security audit, license/policy deny, and coverage.
- No release, upload, package-publish, or signing action was performed.
