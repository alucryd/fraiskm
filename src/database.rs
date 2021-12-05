use super::model::*;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
use sqlx::prelude::*;
// use sqlx::types::Uuid;
use sqlx::{Acquire, Postgres, Transaction};
use uuid::Uuid;

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
) -> Uuid {
    sqlx::query_as!(
        RowId,
        "
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        RETURNING id",
        username,
        password_hash,
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while creating user with username {}", username))
    .id
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

pub async fn create_person(connection: &mut PgConnection, name: &[u8], user_id: &Uuid) -> Uuid {
    sqlx::query_as!(
        RowId,
        "
        INSERT INTO people (name, user_id)
        VALUES ($1, $2)
        RETURNING id",
        name,
        user_id,
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating person")
    .id
}

pub async fn update_person(connection: &mut PgConnection, id: &Uuid, name: &[u8]) {
    sqlx::query!(
        "
        UPDATE people
        SET name = $2
        WHERE id = $1
        ",
        id,
        name,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating person with id {}", id));
}

pub async fn find_people_by_user_id(connection: &mut PgConnection, user_id: &Uuid) -> Vec<Person> {
    sqlx::query_as!(
        Person,
        "
        SELECT *
        FROM people
        WHERE user_id = $1
        ORDER BY name
        ",
        user_id
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding people with user_id {}", user_id))
}

pub async fn delete_person_by_id(connection: &mut PgConnection, id: &Uuid) {
    sqlx::query!(
        "
        DELETE FROM people
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting person with id {}", id));
}
