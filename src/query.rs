use super::database::*;
use super::model::*;
use super::util::*;
use async_graphql::{ComplexObject, Context, Object, Result};
use async_std::sync::{Arc, Mutex};
use sqlx::postgres::PgPool;
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

#[ComplexObject]
impl UserObject {
    async fn addresses(&self, ctx: &Context<'_>) -> Result<Vec<AddressObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(find_addresses_by_user_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &session.get::<Uuid>("id").unwrap(),
        )
        .await
        .into_iter()
        .map(|address| {
            AddressObject::from_db(
                address,
                ctx.data_unchecked::<RingCryptor>(),
                &decode_key(&session.get::<String>("key").unwrap()),
            )
        })
        .collect())
    }

    async fn cars(&self, ctx: &Context<'_>) -> Result<Vec<CarObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(find_cars_by_user_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &session.get::<Uuid>("id").unwrap(),
        )
        .await
        .into_iter()
        .map(|car| {
            CarObject::from_db(
                car,
                ctx.data_unchecked::<RingCryptor>(),
                &decode_key(&session.get::<String>("key").unwrap()),
            )
        })
        .collect())
    }

    async fn people(&self, ctx: &Context<'_>) -> Result<Vec<PersonObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        Ok(find_people_by_user_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &session.get::<Uuid>("id").unwrap(),
        )
        .await
        .into_iter()
        .map(|person| {
            PersonObject::from_db(
                person,
                ctx.data_unchecked::<RingCryptor>(),
                &decode_key(&session.get::<String>("key").unwrap()),
            )
        })
        .collect())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserObject>> {
        Ok(
            find_users(&mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap())
                .await
                .into_iter()
                .map(|user| UserObject::from_db(user))
                .collect(),
        )
    }

    async fn distance(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<Option<DistanceObject>> {
        Ok(find_distance_by_ids(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
        )
        .await
        .map(|distance| DistanceObject::from_db(distance)))
    }
}
