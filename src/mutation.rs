use crate::database::*;
use crate::model::*;
use crate::util::*;
use crate::validator::*;
use async_graphql::{Context, Error, Object, Result};
use async_std::sync::{Arc, Mutex};
use chrono::NaiveDate;
use sqlx::postgres::PgPool;
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn login(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] username: String,
        password: String,
    ) -> Result<UserObject> {
        if let Some(user) = find_user_by_username(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &username,
        )
        .await
        {
            if hash_password(&password) != user.password_hash {
                return Err(Error::new("invalid password"));
            }
            let mut session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
            session
                .insert("key", derive_pbkdf2_key(&username, &password))
                .unwrap();
            session.insert("id", user.id).unwrap();
            Ok(UserObject::from_db(user))
        } else {
            Err(Error::new("unknown username"))
        }
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        ctx.data_unchecked::<Arc<Mutex<Session>>>()
            .lock()
            .await
            .destroy();
        Ok(true)
    }

    async fn add_user(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "EmailValidator::new()"))] username: String,
        password: String,
    ) -> Result<UserObject> {
        create_user(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &username,
            &blake3::hash(password.as_bytes()).to_hex(),
        )
        .await;
        self.login(ctx, username, password).await
    }

    async fn delete_user(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            delete_user_by_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
            )
            .await;
            self.logout(ctx).await
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn update_password(
        &self,
        ctx: &Context<'_>,
        old_password: String,
        new_password: String,
    ) -> Result<bool> {
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            let user = find_user_by_id(&mut connection, &id).await;
            if hash_password(&old_password) != user.password_hash {
                return Err(Error::new("invalid password"));
            }
            update_user(&mut connection, &id, &hash_password(&new_password)).await;
            Ok(true)
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn normalize_address(&self, ctx: &Context<'_>, label: String) -> Result<Vec<String>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        let client = ctx.data_unchecked::<surf::Client>();
        let request = surf::get("https://api-adresse.data.gouv.fr/search/")
            .query(&AddressRequest { q: label })
            .unwrap()
            .build();
        let mut response = client.send(request).await.unwrap();
        let AddressResponse { features } = response.body_json().await.unwrap();
        Ok(features.into_iter().map(|f| f.properties.label).collect())
    }

    async fn add_address(&self, ctx: &Context<'_>, title: String, label: String) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(create_address(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                    &decode_key(&session.get::<String>("key").unwrap()),
                    title.as_bytes(),
                )?,
                &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                    &decode_key(&session.get::<String>("key").unwrap()),
                    label.as_bytes(),
                )?,
                &id,
            )
            .await)
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn update_address(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        title: String,
        label: String,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_address(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                title.as_bytes(),
            )?,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                label.as_bytes(),
            )?,
        )
        .await)
    }

    async fn delete_address(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(delete_address_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }

    async fn add_driver(
        &self,
        ctx: &Context<'_>,
        name: String,
        limit_distance: bool,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(create_driver(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                    &decode_key(&session.get::<String>("key").unwrap()),
                    name.as_bytes(),
                )?,
                limit_distance,
                &id,
            )
            .await)
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn update_driver(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        name: String,
        limit_distance: bool,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_driver(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                name.as_bytes(),
            )?,
            limit_distance,
        )
        .await)
    }

    async fn delete_driver(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(delete_driver_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }

    async fn add_vehicle(
        &self,
        ctx: &Context<'_>,
        model: String,
        horsepower: i16,
        electric: bool,
        vehicle_type_id: i16,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(create_vehicle(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                    &decode_key(&session.get::<String>("key").unwrap()),
                    model.as_bytes(),
                )?,
                horsepower,
                electric,
                vehicle_type_id,
                &id,
            )
            .await)
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn update_vehicle(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        model: String,
        horsepower: i16,
        electric: bool,
        vehicle_type_id: i16,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_vehicle(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                model.as_bytes(),
            )?,
            horsepower,
            electric,
            vehicle_type_id,
        )
        .await)
    }

    async fn delete_vehicle(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(delete_vehicle_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }

    async fn create_distance(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
        meters: i32,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(create_distance(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
            meters,
        )
        .await)
    }

    async fn update_distance(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
        meters: i32,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_distance(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
            meters,
        )
        .await)
    }

    async fn delete_distance(&self, ctx: &Context<'_>, from_id: Uuid, to_id: Uuid) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(delete_distance_by_ids(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
        )
        .await)
    }

    async fn create_journey(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
        driver_id: Uuid,
        vehicle_id: Uuid,
        date: NaiveDate,
        meters: i32,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        let driver = find_driver_by_id(&mut connection, &driver_id).await;
        if driver.limit_distance {
            let distance = compute_total_distance_by_date_and_driver_id_and_vehicle_id(
                &mut connection,
                &date,
                &driver_id,
                &vehicle_id,
                &None,
            )
            .await;
            if distance + meters as i64 > 80000 {
                return Err(Error::new("distance above daily limit"));
            }
        }
        Ok(create_journey(
            &mut connection,
            &from_id,
            &to_id,
            &driver_id,
            &vehicle_id,
            &date,
            meters,
        )
        .await)
    }

    async fn update_journey(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        from_id: Uuid,
        to_id: Uuid,
        driver_id: Uuid,
        vehicle_id: Uuid,
        date: NaiveDate,
        meters: i32,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        let driver = find_driver_by_id(&mut connection, &driver_id).await;
        if driver.limit_distance {
            let distance = compute_total_distance_by_date_and_driver_id_and_vehicle_id(
                &mut connection,
                &date,
                &driver_id,
                &vehicle_id,
                &Some(id),
            )
            .await;
            if distance + meters as i64 > 80000 {
                return Err(Error::new("distance above daily limit"));
            }
        }
        Ok(update_journey(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &from_id,
            &to_id,
            &driver_id,
            &vehicle_id,
            &date,
            meters,
        )
        .await)
    }

    async fn delete_journey(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(delete_journey_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }
}
