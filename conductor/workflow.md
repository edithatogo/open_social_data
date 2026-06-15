# Project Workflow

## 1. Branching Strategy
- **Primary Branch:** `main` (always deployable, clean build, tests passing).
- **Feature Branches:** Use descriptive names prefixed by feature or fix type:
  - `feature/<name>` for new features or data integrations.
  - `fix/<name>` for bug fixes.
  - `docs/<name>` for documentation updates.
- **Integration Flow:** Development occurs in feature/fix branches. 
  - **Task Commits:** Perform a Git commit after the completion of each individual task.
  - **Phase Pushes & Reviews:** Push changes and perform reviews after the completion of each project Phase.
  - **GitHub Actions Verification:** Check GitHub Actions status and confirm build integrity after the completion of each Track.
  - Merging into `main` must occur via a Pull Request (PR) after passing code reviews and automated verification checks.

## 2. Commit Message Guidelines
All commit messages must follow the Conventional Commits specification:
- `feat(api): add ABS dataset provider`
- `fix(parser): resolve overflow in integer parsing`
- `docs(readme): update contributing guidelines`
- `chore: update dependencies`

## 3. Pull Request & Merging Requirements
Before any PR is merged into `main`, the following checks must pass:
1. **Compilation & Linting:** `cargo check` and `cargo clippy -- -D warnings` must complete with zero errors.
2. **Formatting:** `cargo fmt --check` must pass.
3. **Tests:** All unit and integration tests must pass.
4. **Code Review:** At least one maintainer approval (if in a team environment).

## 4. Release Process
- Tag releases using semantic versioning (e.g., `v1.0.0`).
- Automated workflows will build production release binaries and compile updated parquet datasets for distribution upon creating a new release tag.

