Project details in @README.md

- Do NOT use `--all-features` or `sqlite-unbundled` feature on macOS (system SQLite missing required features).
- Edition 2024 is correct
- You may refer to the clone of sqlx at ~/projects/sqlx when needed
- Before committing: `cargo clippy --workspace --all-targets --features sqlite`, `cargo test --features sqlite`, `cargo fmt --all`
- After query changes: `cargo sqlx prepare --workspace -- --tests --features sqlite`
