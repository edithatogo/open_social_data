# Code Style Guides

## 1. Code Formatting & Linting
- **Tooling:** Always use the standard Rust formatter (`rustfmt`) with default settings.
- **Lints:** Enforce strict linting via `clippy`. Run:
  ```bash
  cargo clippy --all-targets -- -D warnings
  ```
- **Build Warnings:** No warnings allowed in main/master branch commits. Treat all compiler warnings as errors.

## 2. Naming Conventions
- **Files & Directories:** Lower snake_case (e.g., `src/dataset_provider.rs`).
- **Types, Structs, Enums, & Traits:** UpperCamelCase (e.g., `struct AbsProvider`, `trait DatasetProvider`).
- **Functions, Variables, & Modules:** snake_case (e.g., `fn fetch_metadata()`).
- **Constants & Statics:** SCREAMING_SNAKE_CASE (e.g., `const MAX_RETRY_ATTEMPTS: u32 = 5;`).

## 3. Error Handling
- **Libraries & Core Trait Implementations:** Use `thiserror` to define precise, structured, and domain-specific enums.
- **Application Binary & CLI Entrypoints:** Use `anyhow` for simple error propagation and context formatting.
- **Avoid Panics:** Avoid `unwrap()` or `expect()` in library code. Propagate errors to the caller using the `?` operator.

## 4. Testing & Quality Assurance
- **Unit Tests:** Place in the same file as the implementation under a `mod tests` module annotated with `#[cfg(test)]`.
- **Integration Tests:** Place in the `tests/` directory at the project root for end-to-end API fetching and file transformation flows.
- **Mocking:** Mock external agency HTTP responses (using libraries like `wiremock`) to ensure tests are fast, self-contained, and run without internet access.
