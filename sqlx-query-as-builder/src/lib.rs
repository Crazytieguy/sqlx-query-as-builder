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
