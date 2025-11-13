use bon::Builder;
use sqlx::sqlite::SqlitePool;
use sqlx_query_as_builder::{query_as_builder, query_file_as_builder};

#[derive(Debug, PartialEq, Builder)]
struct User {
    id: i64,
    name: String,
    email: String,
    age: Option<i64>,
    role: Option<String>,
    #[builder(default = false)]
    verified: bool,
}

#[sqlx::test(fixtures("users"))]
async fn test_query_as_builder_fetch_one(pool: SqlitePool) -> sqlx::Result<()> {
    let user_id = 1_i64;
    let user = query_as_builder!(
        User::builder(),
        r#"SELECT id, name, email, age as "maybe_age" FROM users WHERE id = ?"#,
        user_id
    )
    .fetch_one(&pool)
    .await?
    .role("admin".to_string())
    .verified(true)
    .build();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.email, "alice@example.com");
    assert_eq!(user.age, Some(30));
    assert_eq!(user.role, Some("admin".to_string()));
    assert!(user.verified);

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn test_query_as_builder_fetch_all(pool: SqlitePool) -> sqlx::Result<()> {
    let users: Vec<User> = query_as_builder!(
        User::builder(),
        r#"SELECT id, name, email, age as "maybe_age" FROM users"#
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|builder| builder.role("user".to_string()).verified(false).build())
    .collect();

    assert_eq!(users.len(), 3);

    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[0].role, Some("user".to_string()));
    assert!(!users[0].verified);

    assert_eq!(users[1].name, "Bob");
    assert_eq!(users[1].age, None);
    assert_eq!(users[1].role, Some("user".to_string()));

    assert_eq!(users[2].name, "Charlie");
    assert_eq!(users[2].age, Some(25));

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn test_query_as_builder_with_filter(pool: SqlitePool) -> sqlx::Result<()> {
    let min_age = 25_i64;
    let users: Vec<User> = query_as_builder!(
        User::builder(),
        r#"SELECT id, name, email, age as "maybe_age" FROM users WHERE age >= ?"#,
        min_age
    )
    .fetch_all(&pool)
    .await?
    .into_iter()
    .map(|builder| builder.verified(true).build())
    .collect();

    assert_eq!(users.len(), 2);
    assert!(users.iter().all(|u| u.verified));
    assert!(users.iter().all(|u| u.age.is_some()));
    assert!(users.iter().all(|u| u.age.unwrap() >= 25));

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn test_query_as_builder_partial_fields(pool: SqlitePool) -> sqlx::Result<()> {
    let user_id = 2_i64;
    let user = query_as_builder!(
        User::builder(),
        r#"SELECT id, name, email, age as "maybe_age" FROM users WHERE id = ?"#,
        user_id
    )
    .fetch_one(&pool)
    .await?
    .build();

    assert_eq!(user.id, 2);
    assert_eq!(user.name, "Bob");
    assert_eq!(user.age, None);
    assert_eq!(user.role, None);
    assert!(!user.verified);

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn test_query_file_as_builder(pool: SqlitePool) -> sqlx::Result<()> {
    let user_id = 1_i64;
    let user = query_file_as_builder!(User::builder(), "tests/queries/get_user.sql", user_id)
        .fetch_one(&pool)
        .await?
        .role("moderator".to_string())
        .verified(true)
        .build();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Alice");
    assert_eq!(user.role, Some("moderator".to_string()));
    assert!(user.verified);

    Ok(())
}
