use crate::database::*;
use crate::model::*;
use async_graphql::{Context, Error, Object, Result};
use async_std::sync::{Arc, Mutex};
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use itertools::Itertools;
use sqlx::postgres::PgPool;
use std::cmp::min;
use tide::sessions::Session;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

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
                    &session.get::<String>("key").unwrap(),
                )
                .unwrap()
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
                    &session.get::<String>("key").unwrap(),
                )
                .unwrap()
            })
            .collect())
        } else {
            Err(Error::new("not logged in"))
        }
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
                    &session.get::<String>("key").unwrap(),
                )
                .unwrap()
            })
            .collect())
        } else {
            Err(Error::new("not logged in"))
        }
    }

    async fn distance(
        &self,
        ctx: &Context<'_>,
        id0: Uuid,
        id1: Uuid,
    ) -> Result<Option<DistanceObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        if session.get::<Uuid>("id").is_none() {
            return Err(Error::new("not logged in"));
        }
        Ok(find_distance_by_ids(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            (&id0, &id1),
        )
        .await
        .map(DistanceObject::from_db))
    }

    async fn journeys(
        &self,
        ctx: &Context<'_>,
        driver_id: Uuid,
        year: i16,
        month: i8,
    ) -> Result<Vec<JourneyObject>> {
        Ok(find_journeys_by_driver_id_and_year_and_month(
            &mut ctx.data_unchecked::<PgPool>().acquire().await.unwrap(),
            &driver_id,
            year,
            month,
        )
        .await
        .into_iter()
        .map(JourneyObject::from_db)
        .collect())
    }

    async fn totals(
        &self,
        ctx: &Context<'_>,
        year: i16,
        driver_id: Uuid,
    ) -> Result<Vec<TotalObject>> {
        let session = ctx.data_unchecked::<Arc<Mutex<Session>>>().lock().await;
        let user_id = session.get::<Uuid>("id");
        if user_id.is_none() {
            return Err(Error::new("not logged in"));
        }
        let mut totals = Vec::new();
        let mut connection = ctx.data_unchecked::<PgPool>().acquire().await.unwrap();
        let driver = find_driver_by_id(&mut connection, &driver_id).await;
        let vehicles = find_vehicles_by_user_id(&mut connection, &user_id.unwrap()).await;
        for vehicle in vehicles {
            let scale = find_scale_by_year_and_horsepower_and_vehicle_type(
                &mut connection,
                year + 1,
                vehicle.horsepower,
                vehicle.vehicle_type,
            )
            .await;
            let journeys =
                find_journeys_by_driver_id_and_year(&mut connection, &driver_id, year).await;
            let mut distance: i64 = 0;
            for (_, journeys) in &journeys.into_iter().group_by(|journey| journey.date) {
                let meters: i32 = journeys
                    .map(|journey| {
                        if journey.round_trip {
                            journey.meters * 2
                        } else {
                            journey.meters
                        }
                    })
                    .sum();
                distance += if driver.limit_distance {
                    min(meters, 80000) as i64
                } else {
                    meters as i64
                };
            }
            if distance == 0 {
                continue;
            }
            let mut formula;
            let mut total = BigDecimal::zero();
            let first_threshold = scale.first_threshold as i64 * 1000;
            let second_threshold = scale.second_threshold as i64 * 1000;
            if distance < first_threshold as i64 {
                formula = format!("{} x {}", distance / 1000, scale.first_slice_multiplier);
                total += scale.first_slice_multiplier * BigDecimal::from(distance / 1000);
            } else if distance < second_threshold as i64 {
                formula = format!(
                    "{} x {} + {}",
                    distance / 1000,
                    scale.second_slice_multiplier,
                    scale.second_slice_fixed_amount
                );
                total += scale.second_slice_multiplier * BigDecimal::from(distance / 1000)
                    + BigDecimal::from(scale.second_slice_fixed_amount)
            } else {
                formula = format!("{} x {}", distance / 1000, scale.third_slice_multiplier);
                total += scale.third_slice_multiplier * BigDecimal::from(distance / 1000);
            }
            if vehicle.electric {
                total += &total / BigDecimal::from(5);
                formula = format!("({}) x 1.20", &formula);
            }
            totals.push(TotalObject {
                vehicle_id: vehicle.id,
                distance,
                formula,
                total: total.to_i64().unwrap(),
            });
        }
        Ok(totals)
    }
}
