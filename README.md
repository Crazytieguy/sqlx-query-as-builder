# sqlx-query-as-builder

Builder pattern macros for SQLx `query_as!` - populate builders instead of constructing structs directly.

## Overview

Provides `query_as_builder!` macros that work like SQLx's `query_as!`, but instead of constructing a struct directly, they populate a builder pattern. This allows you to set additional fields or apply transformations before building the final struct.

## Example

```rust
use sqlx_query_as_builder::query_as_builder;

let user = query_as_builder!(
    User::builder(),
    "SELECT id, name FROM users WHERE id = ?",
    user_id
)
.fetch_one(&pool)
.await?
.role("admin")           // Set additional fields
.verified(true)          //
.build()?;               // Build final struct
```

The builder expression must have setter methods matching the SQL column names. Works with any builder library (bon, derive_builder, typed-builder) or hand-written builders.

## Installation

```toml
[dependencies]
sqlx = { version = "=0.8.6", features = ["<your-db>", "<runtime>"] }
sqlx-query-as-builder = { version = "0.1.2", features = ["<your-db>"] }
```

You must pin SQLx to exactly version 0.8.6 and enable the same database features on both `sqlx` and `sqlx-query-as-builder`.

Supported features: `mysql`, `postgres`, `sqlite`, `sqlite-unbundled`, and all SQLx type integrations (`chrono`, `uuid`, `json`, etc.)

## Macros

- `query_as_builder!(builder_expr, "SQL", args...)` - Checked query
- `query_as_builder_unchecked!(...)` - Unchecked variant
- `query_file_as_builder!(...)` - Load SQL from file
- `query_file_as_builder_unchecked!(...)` - Unchecked file variant

## Implementation

Leverages `sqlx_macros_core::query::expand_input()` and transforms the output to replace struct construction with builder method calls. Full feature parity with SQLx's `query_as!` macros.

## Testing

See [integration tests](sqlx-query-as-builder/tests/integration_test.rs) for usage examples.

Run tests with: `cargo test --features sqlite`

## License

Licensed under either of Apache License 2.0 or MIT license at your option.
