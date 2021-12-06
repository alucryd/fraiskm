use super::model::*;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
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
    println!("{}", username);
    println!("{}", password_hash);
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

pub async fn update_user(connection: &mut PgConnection, id: &Uuid, password_hash: &str) -> u64 {
    sqlx::query!(
        "
        UPDATE users
        SET password_hash = $2
        WHERE id = $1
        ",
        id,
        password_hash,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating user with id {}", id))
    .rows_affected()
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

pub async fn find_user_by_id(connection: &mut PgConnection, id: &Uuid) -> User {
    sqlx::query_as!(
        User,
        "
        SELECT *
        FROM users
        WHERE id = $1
        ",
        id,
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding user with id {}", id))
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

pub async fn delete_user_by_username(connection: &mut PgConnection, username: &str) -> u64 {
    sqlx::query!(
        "
        DELETE FROM users
        WHERE username = $1
        ",
        username,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting user with username {}", username))
    .rows_affected()
}

pub async fn create_address(
    connection: &mut PgConnection,
    title: &[u8],
    label: &[u8],
    user_id: &Uuid,
) -> Uuid {
    sqlx::query_as!(
        RowId,
        "
        INSERT INTO addresses (title, label, user_id)
        VALUES ($1, $2, $3)
        RETURNING id",
        title,
        label,
        user_id,
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating address")
    .id
}

pub async fn update_address(
    connection: &mut PgConnection,
    id: &Uuid,
    title: &[u8],
    label: &[u8],
) -> u64 {
    sqlx::query!(
        "
        UPDATE addresses
        SET title = $2, label = $3
        WHERE id = $1
        ",
        id,
        title,
        label,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating address with id {}", id))
    .rows_affected()
}

pub async fn find_addresses_by_user_id(
    connection: &mut PgConnection,
    user_id: &Uuid,
) -> Vec<Address> {
    sqlx::query_as!(
        Address,
        "
        SELECT *
        FROM addresses
        WHERE user_id = $1
        ORDER BY title
        ",
        user_id
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding addresses with user_id {}", user_id))
}

pub async fn delete_address_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM addresses
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting address with id {}", id))
    .rows_affected()
}

pub async fn create_car(
    connection: &mut PgConnection,
    model: &[u8],
    horsepower: i16,
    user_id: &Uuid,
) -> Uuid {
    sqlx::query_as!(
        RowId,
        "
        INSERT INTO cars (model, horsepower, user_id)
        VALUES ($1, $2, $3)
        RETURNING id",
        model,
        horsepower,
        user_id,
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating car")
    .id
}

pub async fn update_car(
    connection: &mut PgConnection,
    id: &Uuid,
    model: &[u8],
    horsepower: i16,
) -> u64 {
    sqlx::query!(
        "
        UPDATE cars
        SET model = $2, horsepower = $3
        WHERE id = $1
        ",
        id,
        model,
        horsepower,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating car with id {}", id))
    .rows_affected()
}

pub async fn find_cars_by_user_id(connection: &mut PgConnection, user_id: &Uuid) -> Vec<Car> {
    sqlx::query_as!(
        Car,
        "
        SELECT *
        FROM cars
        WHERE user_id = $1
        ORDER BY model
        ",
        user_id
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding cars with user_id {}", user_id))
}

pub async fn delete_car_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM cars
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting car with id {}", id))
    .rows_affected()
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

pub async fn update_person(connection: &mut PgConnection, id: &Uuid, name: &[u8]) -> u64 {
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
    .unwrap_or_else(|_| panic!("Error while updating person with id {}", id))
    .rows_affected()
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

pub async fn delete_person_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM people
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting person with id {}", id))
    .rows_affected()
}

pub async fn create_distance(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
    meters: i32,
) -> u64 {
    sqlx::query!(
        "
        INSERT INTO distances (from_id, to_id, meters)
        VALUES ($1, $2, $3)",
        from_id,
        to_id,
        meters,
    )
    .execute(connection)
    .await
    .expect("Error while creating distance")
    .rows_affected()
}

pub async fn update_distance(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
    meters: i32,
) -> u64 {
    sqlx::query!(
        "
        UPDATE distances
        SET meters = $3
        WHERE from_id = $1
        AND to_id = $2
        ",
        from_id,
        to_id,
        meters,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while updating distance with ids {} and {}",
            from_id, to_id
        )
    })
    .rows_affected()
}

pub async fn find_distance_by_ids(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
) -> Option<Distance> {
    sqlx::query_as!(
        Distance,
        "
        SELECT *
        FROM distances
        WHERE from_id = $1
        AND to_id = $2
        ",
        from_id,
        to_id,
    )
    .fetch_optional(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while finding distance with ids {} and {}",
            from_id, to_id
        )
    })
}

pub async fn delete_distance_by_ids(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
) -> u64 {
    sqlx::query!(
        "
        DELETE FROM distances
        WHERE from_id = $1
        AND to_id = $2
        ",
        from_id,
        to_id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while deleting distances with ids {} and {}",
            from_id, to_id
        )
    })
    .rows_affected()
}
