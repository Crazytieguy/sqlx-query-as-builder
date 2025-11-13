# sqlx-query-as-builder: Design Document

## Overview

Create a crate that provides `query_as_builder!` macros - variants of sqlx's `query_as!` that populate a builder pattern instead of constructing a struct directly.

## API

```rust
// User has a builder with setters
let record = query_as_builder!(
    MyStruct::builder(),
    "SELECT id, name FROM users WHERE id = ?",
    user_id
)
.fetch_one(&pool)
.await?
.additional_field(some_value)  // Can set more fields before building
.build()?;
```

The builder expression can be a variable or expression - both work.

## Crate Structure

Follow sqlx's pattern with two crates:

**`sqlx-query-as-builder`** (top-level):
- Declarative macros that provide nice syntax: `query_as_builder!(builder, "SELECT ...")`
- Delegates to proc-macro crate

**`sqlx-query-as-builder-macros`** (proc-macro):
- Contains `expand_query_as_builder` proc-macro
- Dependencies: `sqlx-macros-core` (pinned exact version), `sqlx-core` (pinned for version alignment), `syn`, `quote`, `proc-macro2`

## Implementation Strategy

**Key insight:** We don't duplicate any sqlx internals. Instead:

1. Parse our macro input to extract: builder expression + sqlx query parameters
2. Call `sqlx_macros_core::query::expand_input()` as a black box - this handles all the complexity (type checking, overrides, describe data, etc.)
3. Parse the output TokenStream to find the final return expression: `Result::Ok(MyStruct { field0: var0, field1: var1 })`
4. Replace it with: `Result::Ok(builder_expr.field0(var0).field1(var1))`
5. Return the modified TokenStream

This gives us full feature parity with sqlx (type overrides, nullability, checked/unchecked variants) for ~50-70 lines of code.

## Parsing Logic

Use `syn::visit_mut::VisitMut` to walk the expression tree and find the struct literal construction inside `Result::Ok(...)`. The struct literal pattern is stable across both checked and unchecked variants - the only difference is in the variable binding types, which we don't touch.

Reference implementation pattern in sqlx/sqlx-macros-core/src/query/output.rs:195-203 - that's what we're finding and replacing.

## Variants to Support

Match sqlx's full API surface:
- `query_as_builder!` / `query_as_builder_unchecked!`
- `query_file_as_builder!` / `query_file_as_builder_unchecked!`

All variants produce the same struct literal pattern in their output, so one transformation handles everything.

## Features

Mirror sqlx's database feature flags (`postgres`, `mysql`, `sqlite`) since `FOSS_DRIVERS` in `sqlx-macros-core` is conditionally compiled. Users must enable the same features they use with sqlx:

```toml
sqlx-query-as-builder = { version = "0.1", features = ["postgres"] }
```
See sqlx/Cargo.toml and sqlx/sqlx-macros/Cargo.toml for full details
