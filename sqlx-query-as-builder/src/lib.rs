//! Builder pattern macros for SQLx `query_as!` - populate builders instead of constructing structs directly.
//!
//! The builder expression must have setter methods matching the SQL column names.

/// Like `query_as!` but populates a builder instead of constructing a struct directly.
///
/// # Example
/// ```ignore
/// let user = query_as_builder!(User::builder(), "SELECT id, name FROM users WHERE id = ?", user_id)
///     .fetch_one(&pool)
///     .await?
///     .role("admin")
///     .build();
/// ```
#[macro_export]
macro_rules! query_as_builder {
    ($builder_expr:expr, $query:expr) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source = $query
        )
    }};
    ($builder_expr:expr, $query:expr, $($args:tt)*) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source = $query,
            args = [$($args)*]
        )
    }};
}

/// Unchecked version of `query_as_builder!` - does not verify query at compile time.
#[macro_export]
macro_rules! query_as_builder_unchecked {
    ($builder_expr:expr, $query:expr) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source = $query,
            checked = false
        )
    }};
    ($builder_expr:expr, $query:expr, $($args:tt)*) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source = $query,
            args = [$($args)*],
            checked = false
        )
    }};
}

/// Like `query_file_as!` but populates a builder instead of constructing a struct directly.
#[macro_export]
macro_rules! query_file_as_builder {
    ($builder_expr:expr, $path:literal) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source_file = $path
        )
    }};
    ($builder_expr:expr, $path:literal, $($args:tt)*) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source_file = $path,
            args = [$($args)*]
        )
    }};
}

/// Unchecked version of `query_file_as_builder!` - does not verify query at compile time.
#[macro_export]
macro_rules! query_file_as_builder_unchecked {
    ($builder_expr:expr, $path:literal) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source_file = $path,
            checked = false
        )
    }};
    ($builder_expr:expr, $path:literal, $($args:tt)*) => {{
        $crate::sqlx_query_as_builder_macros::expand_query_as_builder!(
            $builder_expr,
            source_file = $path,
            args = [$($args)*],
            checked = false
        )
    }};
}

#[doc(hidden)]
pub use sqlx_query_as_builder_macros;
