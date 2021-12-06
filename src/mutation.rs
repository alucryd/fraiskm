use super::database::*;
use super::model::*;
use super::util::*;
use async_graphql::{Context, Error, Object, Result};
use async_std::sync::{Arc, Mutex};
use sqlx::postgres::PgPool;
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signup(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<UserObject> {
        let id = create_user(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &username,
            &blake3::hash(password.as_bytes()).to_hex(),
        )
        .await;
        let user = find_user_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await;
        let mut session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        session
            .insert("key", derive_pbkdf2_key(&username, &password))
            .unwrap();
        session.insert("id", id).unwrap();
        Ok(UserObject::from_db(user))
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<UserObject> {
        let user = find_user_by_username(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &username,
        )
        .await;
        if hash_password(&password) != user.password_hash {
            return Err(Error::new("invalid password"));
        }
        let mut session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        session
            .insert("key", derive_pbkdf2_key(&username, &password))
            .unwrap();
        session.insert("id", user.id).unwrap();
        Ok(UserObject::from_db(user))
    }

    async fn add_address(&self, ctx: &Context<'_>, title: String, label: String) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
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
            &session.get::<Uuid>("id").unwrap(),
        )
        .await)
    }

    async fn update_address(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        title: String,
        label: String,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
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
        Ok(delete_address_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }

    async fn add_car(&self, ctx: &Context<'_>, model: String, horsepower: i16) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(create_car(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                model.as_bytes(),
            )?,
            horsepower,
            &session.get::<Uuid>("id").unwrap(),
        )
        .await)
    }

    async fn update_car(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        model: String,
        horsepower: i16,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(update_car(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                model.as_bytes(),
            )?,
            horsepower,
        )
        .await)
    }

    async fn delete_car(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        Ok(delete_car_by_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
        )
        .await)
    }

    async fn add_person(&self, ctx: &Context<'_>, name: String) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(create_person(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                name.as_bytes(),
            )?,
            &session.get::<Uuid>("id").unwrap(),
        )
        .await)
    }

    async fn update_person(&self, ctx: &Context<'_>, id: Uuid, name: String) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(update_person(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &ctx.data_unchecked::<RingCryptor>().seal_with_key(
                &decode_key(&session.get::<String>("key").unwrap()),
                name.as_bytes(),
            )?,
        )
        .await)
    }

    async fn delete_person(&self, ctx: &Context<'_>, id: Uuid) -> Result<u64> {
        Ok(delete_person_by_id(
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
        Ok(update_distance(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
            meters,
        )
        .await)
    }

    async fn delete_distance(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
        meters: i32,
    ) -> Result<u64> {
        Ok(delete_distance_by_ids(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
        )
        .await)
    }
}
