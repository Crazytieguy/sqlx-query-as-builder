Project details in @README.md

- Do NOT use `--all-features` or `sqlite-unbundled` feature on macOS (system SQLite missing required features).
- Edition 2024 is correct
- You may refer to the clone of sqlx at ~/projects/sqlx when needed
- Before committing: `cargo clippy --workspace --all-targets --features sqlite`, `cargo test --features sqlite`, `cargo fmt --all`
- After query changes: `cargo sqlx prepare --workspace -- --tests --features sqlite`
- To release:
  1. Bump patch version in workspace Cargo.toml (keeping 0.1.x while API is unstable)
  2. Update version in installation sections in README.md and sqlx-query-as-builder/src/lib.rs
  3. Commit
  4. `cargo publish -p sqlx-query-as-builder-macros && cargo publish -p sqlx-query-as-builder`
  5. Create and push git tag
- When adding to or changing the crate API surface, update explanations and examples in both README.md and sqlx-query-as-builder/src/lib.rs to reflect the changes
