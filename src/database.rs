use super::model::*;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
use sqlx::prelude::*;
// use sqlx::types::Uuid;
use uuid::Uuid;
use sqlx::{Acquire, Postgres, Transaction};

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn establish_connection(url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", url));

    MIGRATOR
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    pool
}

pub async fn begin_transaction(connection: &mut PgConnection) -> Transaction<'_, Postgres> {
    Acquire::begin(connection)
        .await
        .expect("Failed to begin transaction")
}

pub async fn commit_transaction(transaction: Transaction<'_, Postgres>) {
    transaction
        .commit()
        .await
        .expect("Failed to commit transaction");
}

pub async fn rollback_transaction(transaction: Transaction<'_, Postgres>) {
    transaction
        .rollback()
        .await
        .expect("Failed to rollback transaction");
}

pub async fn create_user(
    connection: &mut PgConnection,
    username: &str,
    password_hash: &str,
) {
    sqlx::query!(
        "
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        ",
        username,
        password_hash,
    )
    .execute(connection)
    .await
    .expect("Error while creating user");
}

pub async fn update_user_password(
    connection: &mut PgConnection,
    username: &str,
    password_hash: &str,
) {
    sqlx::query!(
        "
        UPDATE users
        SET password_hash = $2
        WHERE username = $1
        ",
        username,
        password_hash,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while updating user password with username {}",
            username
        )
    });
}

pub async fn find_users(connection: &mut PgConnection) -> Vec<User> {
    sqlx::query_as!(
        User,
        "
        SELECT *
        FROM users
        ORDER BY username
        ",
    )
    .fetch_all(connection)
    .await
    .expect("Error while finding users")
}

pub async fn find_user_by_username(connection: &mut PgConnection, username: &str) -> User {
    sqlx::query_as!(
        User,
        "
        SELECT *
        FROM users
        WHERE username = $1
        ",
        username,
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding user with username {}", username))
}

pub async fn delete_user_by_username(connection: &mut PgConnection, username: &str) {
    sqlx::query!(
        "
        DELETE FROM users
        WHERE username = $1
        ",
        username,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting user with username {}", username));
}
