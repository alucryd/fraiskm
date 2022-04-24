use crate::database::*;
use crate::model::*;
use crate::util::*;
use crate::validator::*;
use async_graphql::{Context, Error, Object, Result};
use async_std::sync::{Arc, Mutex};
use chrono::NaiveDate;
use log::debug;
use sqlx::postgres::PgPool;
use sqlx::Acquire;
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn signin(
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

    async fn signout(&self, ctx: &Context<'_>) -> Result<bool> {
        ctx.data_unchecked::<Arc<Mutex<Session>>>()
            .lock()
            .await
            .destroy();
        Ok(true)
    }

    async fn signup(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(custom = "EmailValidator::new()"))] username: String,
        #[graphql(validator(custom = "PasswordValidator::new()"))] password: String,
    ) -> Result<UserObject> {
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        if find_user_by_username(&mut connection, &username)
            .await
            .is_none()
        {
            create_user(&mut connection, &username, &hash_password(&password)).await;
            self.signin(ctx, username, password).await
        } else {
            Err(Error::new("username already exists"))
        }
    }

    async fn update_username(
        &self,
        ctx: &Context<'_>,
        #[graphql(validator(email))] new_username: String,
        password: String,
    ) -> Result<bool> {
        debug!("updating username");
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
            let cryptor = ctx.data_unchecked::<RingCryptor>();
            let key = session.get::<String>("key").unwrap();
            let user = find_user_by_id(&mut connection, &id).await;
            if hash_password(&password) != user.password_hash {
                return Err(Error::new("invalid password"));
            }
            if find_user_by_username(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &new_username,
            )
            .await
            .is_none()
            {
                let mut transaction = Acquire::begin(&mut connection)
                    .await
                    .expect("Failed to begin transaction");
                reencrypt_all_data(
                    &mut transaction,
                    cryptor,
                    &key,
                    &id,
                    &new_username,
                    &password,
                )
                .await?;
                update_user_username(&mut transaction, &id, &new_username).await;
                transaction
                    .commit()
                    .await
                    .expect("Failed to commit transaction");
                Ok(true)
            } else {
                Err(Error::new("username already exists"))
            }
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn update_password(
        &self,
        ctx: &Context<'_>,
        password: String,
        #[graphql(validator(custom = "PasswordValidator::new()"))] new_password: String,
    ) -> Result<bool> {
        debug!("updating password");
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
            let cryptor = ctx.data_unchecked::<RingCryptor>();
            let key = session.get::<String>("key").unwrap();
            let user = find_user_by_id(&mut connection, &id).await;
            if hash_password(&password) != user.password_hash {
                return Err(Error::new("invalid password"));
            }
            let mut transaction = Acquire::begin(&mut connection)
                .await
                .expect("Failed to begin transaction");
            reencrypt_all_data(
                &mut transaction,
                cryptor,
                &key,
                &id,
                &user.username,
                &new_password,
            )
            .await?;
            update_user_password_hash(&mut transaction, &id, &hash_password(&new_password)).await;
            transaction
                .commit()
                .await
                .expect("Failed to commit transaction");
            Ok(true)
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>) -> Result<bool> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            delete_user_by_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
            )
            .await;
            self.signout(ctx).await
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

    async fn create_address(
        &self,
        ctx: &Context<'_>,
        title: String,
        label: String,
        address_type: i16,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            let cryptor = ctx.data_unchecked::<RingCryptor>();
            let key = session.get::<String>("key").unwrap();
            Ok(create_address(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &encrypt_data(cryptor, &key, &title)?,
                &encrypt_data(cryptor, &key, &label)?,
                address_type,
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
        address_type: i16,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        let cryptor = ctx.data_unchecked::<RingCryptor>();
        let key = session.get::<String>("key").unwrap();
        Ok(update_address(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &encrypt_data(cryptor, &key, &title)?,
            &encrypt_data(cryptor, &key, &label)?,
            address_type,
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

    async fn create_vehicle(
        &self,
        ctx: &Context<'_>,
        model: String,
        horsepower: i16,
        electric: bool,
        vehicle_type: i16,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(create_vehicle(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &encrypt_data(
                    ctx.data_unchecked::<RingCryptor>(),
                    &session.get::<String>("key").unwrap(),
                    &model,
                )?,
                horsepower,
                electric,
                vehicle_type,
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
        vehicle_type: i16,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_vehicle(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &encrypt_data(
                ctx.data_unchecked::<RingCryptor>(),
                &session.get::<String>("key").unwrap(),
                &model,
            )?,
            horsepower,
            electric,
            vehicle_type,
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

    async fn create_driver(
        &self,
        ctx: &Context<'_>,
        name: String,
        limit_distance: bool,
        default_vehicle_id: Option<Uuid>,
        default_from_id: Option<Uuid>,
        default_to_id: Option<Uuid>,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(create_driver(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &encrypt_data(
                    ctx.data_unchecked::<RingCryptor>(),
                    &session.get::<String>("key").unwrap(),
                    &name,
                )?,
                limit_distance,
                &id,
                default_vehicle_id.as_ref(),
                default_from_id.as_ref(),
                default_to_id.as_ref(),
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
        default_vehicle_id: Option<Uuid>,
        default_from_id: Option<Uuid>,
        default_to_id: Option<Uuid>,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(update_driver(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &id,
            &encrypt_data(
                ctx.data_unchecked::<RingCryptor>(),
                &session.get::<String>("key").unwrap(),
                &name,
            )?,
            limit_distance,
            default_vehicle_id.as_ref(),
            default_from_id.as_ref(),
            default_to_id.as_ref(),
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

    async fn create_or_update_distance(
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
        Ok(create_or_update_distance(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
            meters,
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
        round_trip: bool,
    ) -> Result<Uuid> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(create_journey(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
            &driver_id,
            &vehicle_id,
            &date,
            meters,
            round_trip,
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
        round_trip: bool,
    ) -> Result<u64> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
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
            round_trip,
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
