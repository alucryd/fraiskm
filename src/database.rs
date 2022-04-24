use crate::model::*;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnection, PgPool, PgPoolOptions};
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

pub async fn create_user(
    connection: &mut PgConnection,
    username: &str,
    password_hash: &str,
) -> Uuid {
    sqlx::query_as!(
        RowUuid,
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

pub async fn update_user_username(connection: &mut PgConnection, id: &Uuid, username: &str) -> u64 {
    sqlx::query!(
        "
        UPDATE users
        SET username = $2
        WHERE id = $1
        ",
        id,
        username,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating user with id {}", id))
    .rows_affected()
}

pub async fn update_user_password_hash(
    connection: &mut PgConnection,
    id: &Uuid,
    password_hash: &str,
) -> u64 {
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

pub async fn find_user_by_username(connection: &mut PgConnection, username: &str) -> Option<User> {
    sqlx::query_as!(
        User,
        "
        SELECT *
        FROM users
        WHERE username = $1
        ",
        username,
    )
    .fetch_optional(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding user with username {}", username))
}

pub async fn delete_user_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM users
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting user with id {}", id))
    .rows_affected()
}

pub async fn create_address(
    connection: &mut PgConnection,
    title: &[u8],
    label: &[u8],
    address_type: i16,
    user_id: &Uuid,
) -> Uuid {
    sqlx::query_as!(
        RowUuid,
        "
        INSERT INTO addresses (title, label, address_type, user_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id",
        title,
        label,
        address_type,
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
    address_type: i16,
) -> u64 {
    sqlx::query!(
        "
        UPDATE addresses
        SET title = $2, label = $3, address_type = $4
        WHERE id = $1
        ",
        id,
        title,
        label,
        address_type,
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
        ORDER BY address_type, title
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

pub async fn create_vehicle(
    connection: &mut PgConnection,
    model: &[u8],
    horsepower: i16,
    electric: bool,
    vehicle_type: i16,
    user_id: &Uuid,
) -> Uuid {
    sqlx::query_as!(
        RowUuid,
        "
        INSERT INTO vehicles (model, horsepower, electric, vehicle_type, user_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id",
        model,
        horsepower,
        electric,
        vehicle_type,
        user_id,
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating vehicle")
    .id
}

pub async fn update_vehicle(
    connection: &mut PgConnection,
    id: &Uuid,
    model: &[u8],
    horsepower: i16,
    electric: bool,
    vehicle_type: i16,
) -> u64 {
    sqlx::query!(
        "
        UPDATE vehicles
        SET model = $2, horsepower = $3, electric = $4, vehicle_type = $5
        WHERE id = $1
        ",
        id,
        model,
        horsepower,
        electric,
        vehicle_type,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating vehicle with id {}", id))
    .rows_affected()
}

pub async fn find_vehicle_by_id(connection: &mut PgConnection, id: &Uuid) -> Vehicle {
    sqlx::query_as!(
        Vehicle,
        "
        SELECT *
        FROM vehicles
        WHERE id = $1
        ",
        id
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding vehicle with id {}", id))
}

pub async fn find_vehicles_by_user_id(
    connection: &mut PgConnection,
    user_id: &Uuid,
) -> Vec<Vehicle> {
    sqlx::query_as!(
        Vehicle,
        "
        SELECT *
        FROM vehicles
        WHERE user_id = $1
        ORDER BY vehicle_type, model
        ",
        user_id
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding vehicles with user_id {}", user_id))
}

pub async fn delete_vehicle_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM vehicles
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting vehicle with id {}", id))
    .rows_affected()
}

pub async fn create_driver(
    connection: &mut PgConnection,
    name: &[u8],
    limit_distance: bool,
    user_id: &Uuid,
    default_vehicle_id: Option<&Uuid>,
    default_from_id: Option<&Uuid>,
    default_to_id: Option<&Uuid>,
) -> Uuid {
    sqlx::query_as!(
        RowUuid,
        "
        INSERT INTO drivers (name, limit_distance, user_id, default_vehicle_id, default_from_id, default_to_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id",
        name,
        limit_distance,
        user_id,
        default_vehicle_id,
        default_from_id,
        default_to_id,
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating driver")
    .id
}

pub async fn update_driver(
    connection: &mut PgConnection,
    id: &Uuid,
    name: &[u8],
    limit_distance: bool,
    default_vehicle_id: Option<&Uuid>,
    default_from_id: Option<&Uuid>,
    default_to_id: Option<&Uuid>,
) -> u64 {
    sqlx::query!(
        "
        UPDATE drivers
        SET name = $2, limit_distance = $3, default_vehicle_id = $4, default_from_id = $5, default_to_id = $6
        WHERE id = $1
        ",
        id,
        name,
        limit_distance,
        default_vehicle_id,
        default_from_id, default_to_id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating driver with id {}", id))
    .rows_affected()
}

pub async fn find_driver_by_id(connection: &mut PgConnection, id: &Uuid) -> Driver {
    sqlx::query_as!(
        Driver,
        "
        SELECT *
        FROM drivers
        WHERE id = $1
        ",
        id
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding driver with id {}", id))
}

pub async fn find_drivers_by_user_id(connection: &mut PgConnection, user_id: &Uuid) -> Vec<Driver> {
    sqlx::query_as!(
        Driver,
        "
        SELECT *
        FROM drivers
        WHERE user_id = $1
        ORDER BY name
        ",
        user_id
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while finding drivers with user_id {}", user_id))
}

pub async fn delete_driver_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM drivers
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting driver with id {}", id))
    .rows_affected()
}

pub async fn create_or_update_distance(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
    meters: i32,
) -> u64 {
    sqlx::query!(
        "
        INSERT INTO distances (from_id, to_id, meters)
        VALUES ($1, $2, $3)
        ON CONFLICT (from_id, to_id)
        DO UPDATE SET meters = $4",
        from_id,
        to_id,
        meters,
        meters,
    )
    .execute(connection)
    .await
    .expect("Error while creating distance")
    .rows_affected()
}

pub async fn find_distance_by_ids(
    connection: &mut PgConnection,
    ids: (&Uuid, &Uuid),
) -> Option<Distance> {
    sqlx::query_as!(
        Distance,
        "
        SELECT *
        FROM distances
        WHERE from_id = $1
        AND to_id = $2
        OR to_id = $1
        AND from_id = $2
        ",
        ids.0,
        ids.1,
    )
    .fetch_optional(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while finding distance with ids {} and {}",
            ids.0, ids.1
        )
    })
}

pub async fn create_journey(
    connection: &mut PgConnection,
    from_id: &Uuid,
    to_id: &Uuid,
    driver_id: &Uuid,
    vehicle_id: &Uuid,
    date: &NaiveDate,
    meters: i32,
    round_trip: bool,
) -> Uuid {
    sqlx::query!(
        "
        INSERT INTO journeys (from_id, to_id, driver_id, vehicle_id, date, meters, round_trip)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id",
        from_id,
        to_id,
        driver_id,
        vehicle_id,
        date,
        meters,
        round_trip
    )
    .fetch_one(connection)
    .await
    .expect("Error while creating journey")
    .id
}

pub async fn update_journey(
    connection: &mut PgConnection,
    id: &Uuid,
    from_id: &Uuid,
    to_id: &Uuid,
    driver_id: &Uuid,
    vehicle_id: &Uuid,
    date: &NaiveDate,
    meters: i32,
    round_trip: bool,
) -> u64 {
    sqlx::query!(
        "
        UPDATE journeys
        SET from_id = $2, to_id = $3, driver_id = $4, vehicle_id = $5, date = $6, meters = $7, round_trip = $8
        WHERE id = $1
        ",
        id,
        from_id,
        to_id,
        driver_id,
        vehicle_id,
        date,
        meters,
        round_trip
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while updating journey with id {}", id))
    .rows_affected()
}

pub async fn find_journeys_by_driver_id_and_year(
    connection: &mut PgConnection,
    driver_id: &Uuid,
    year: i16,
) -> Vec<Journey> {
    sqlx::query_as!(
        Journey,
        "
        SELECT *
        FROM journeys
        WHERE driver_id = $1
        AND extract(year FROM date) = $2
        ORDER BY date
        ",
        driver_id,
        BigDecimal::from(year),
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while finding journeys with driver id {} and year {}",
            driver_id, year
        )
    })
}

pub async fn find_journeys_by_driver_id_and_year_and_month(
    connection: &mut PgConnection,
    driver_id: &Uuid,
    year: i16,
    month: i8,
) -> Vec<Journey> {
    sqlx::query_as!(
        Journey,
        "
        SELECT *
        FROM journeys
        WHERE driver_id = $1
        AND extract(year FROM date) = $2
        AND extract(month FROM date) = $3
        ORDER BY date
        ",
        driver_id,
        BigDecimal::from(year),
        BigDecimal::from(month),
    )
    .fetch_all(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while finding journeys with driver id {} and year {} and month {}",
            driver_id, year, month
        )
    })
}

pub async fn delete_journey_by_id(connection: &mut PgConnection, id: &Uuid) -> u64 {
    sqlx::query!(
        "
        DELETE FROM journeys
        WHERE id = $1
        ",
        id,
    )
    .execute(connection)
    .await
    .unwrap_or_else(|_| panic!("Error while deleting journey with id {}", id))
    .rows_affected()
}

pub async fn find_scale_by_year_and_horsepower_and_vehicle_type(
    connection: &mut PgConnection,
    year: i16,
    horsepower: i16,
    vehicle_type: i16,
) -> Scale {
    sqlx::query_as!(
        Scale,
        "
        SELECT *
        FROM scales
        WHERE year = $1
        AND (
            horsepower_min <= $2 OR horsepower_min IS NULL
        ) AND (
            horsepower_max >= $2 OR horsepower_max IS NULL
        ) AND vehicle_type = $3
        ",
        year,
        horsepower,
        vehicle_type,
    )
    .fetch_one(connection)
    .await
    .unwrap_or_else(|_| {
        panic!(
            "Error while finding scale with year {} and horsepower {} and vehicle type id {}",
            year, horsepower, vehicle_type
        )
    })
}
