use crate::database::*;
use crate::model::*;
use crate::util::*;
use async_graphql::{ComplexObject, Context, Error, Object, Result};
use async_std::sync::{Arc, Mutex};
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use sqlx::postgres::PgPool;
use std::cmp::{max, min};
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

#[ComplexObject]
impl UserObject {}

#[ComplexObject]
impl DriverObject {
    async fn journeys(&self, ctx: &Context<'_>, year: i16) -> Result<Vec<JourneyObject>> {
        Ok(find_journeys_by_year_and_driver_id(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            year,
            &self.id,
        )
        .await
        .into_iter()
        .map(|journey| JourneyObject::from_db(journey))
        .collect())
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn me(&self, ctx: &Context<'_>) -> Result<UserObject> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            let user = find_user_by_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
            )
            .await;
            Ok(UserObject::from_db(user))
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn addresses(&self, ctx: &Context<'_>) -> Result<Vec<AddressObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(find_addresses_by_user_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
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
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn drivers(&self, ctx: &Context<'_>) -> Result<Vec<DriverObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(find_drivers_by_user_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
            )
            .await
            .into_iter()
            .map(|driver| {
                DriverObject::from_db(
                    driver,
                    ctx.data_unchecked::<RingCryptor>(),
                    &decode_key(&session.get::<String>("key").unwrap()),
                )
            })
            .collect())
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn vehicle_types(&self, ctx: &Context<'_>) -> Result<Vec<VehicleTypeObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(
            find_vehicle_types(&mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap())
                .await
                .into_iter()
                .map(|vehicle_type| VehicleTypeObject::from_db(vehicle_type))
                .collect(),
        )
    }

    async fn vehicles(&self, ctx: &Context<'_>) -> Result<Vec<VehicleObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if let Some(id) = session.get::<Uuid>("id") {
            Ok(find_vehicles_by_user_id(
                &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
                &id,
            )
            .await
            .into_iter()
            .map(|vehicle| {
                VehicleObject::from_db(
                    vehicle,
                    ctx.data_unchecked::<RingCryptor>(),
                    &decode_key(&session.get::<String>("key").unwrap()),
                )
            })
            .collect())
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn distance(
        &self,
        ctx: &Context<'_>,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<Option<DistanceObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(find_distance_by_ids(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &from_id,
            &to_id,
        )
        .await
        .map(|distance| DistanceObject::from_db(distance)))
    }

    async fn total(
        &self,
        ctx: &Context<'_>,
        year: i16,
        driver_id: Uuid,
        vehicle_id: Uuid,
    ) -> Result<TotalObject> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        let vehicle = find_vehicle_by_id(&mut connection, &vehicle_id).await;
        let scale = find_scale_by_year_and_horsepower_and_vehicle_type_id(
            &mut connection,
            year,
            vehicle.horsepower,
            vehicle.vehicle_type_id,
        )
        .await;
        let distance = compute_total_distance_by_year_and_driver_id_and_vehicle_id(
            &mut connection,
            year,
            &driver_id,
            &vehicle_id,
        )
        .await;
        let mut formula = String::new();
        let mut total = BigDecimal::zero();
        let first_threshold = scale.first_threshold as i64 * 1000;
        let second_threshold = scale.second_threshold as i64 * 1000;
        let first_slice = min(distance, first_threshold);
        formula += &format!("{} x {}", first_slice / 1000, scale.first_slice_multiplier);
        total += scale.first_slice_multiplier * BigDecimal::from(first_slice);
        if distance > first_threshold as i64 {
            let second_slice = max(
                distance - first_threshold,
                second_threshold - first_threshold,
            );
            formula += &format!(
                " + {} x {} + {}",
                second_slice / 1000,
                scale.second_slice_multiplier,
                scale.second_slice_fixed_amount
            );
            total += scale.second_slice_multiplier * BigDecimal::from(second_slice)
                + BigDecimal::from(scale.second_slice_fixed_amount)
        }
        if distance > second_threshold {
            let third_slice = distance - second_threshold;
            formula += &format!(
                " + {} x {}",
                third_slice / 1000,
                scale.third_slice_multiplier
            );
            total += scale.third_slice_multiplier * BigDecimal::from(third_slice)
        }
        if vehicle.electric {
            total += &total / BigDecimal::from(5);
        }
        total = total / BigDecimal::from(10);
        Ok(TotalObject {
            formula,
            total: total.to_i64().unwrap(),
        })
    }
}
