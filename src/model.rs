use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

#[derive(FromRow)]
pub struct RowUuid {
    pub id: Uuid,
}

#[derive(FromRow)]
pub struct RowInteger {
    pub total: Option<i64>,
}

#[derive(FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct UserObject {
    pub username: String,
}

impl UserObject {
    pub fn from_db(user: User) -> UserObject {
        UserObject {
            username: user.username,
        }
    }
}

#[derive(FromRow)]
pub struct Address {
    pub id: Uuid,
    pub title: Vec<u8>,
    pub label: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(SimpleObject)]
pub struct AddressObject {
    pub id: Uuid,
    pub title: String,
    pub label: String,
}

impl AddressObject {
    pub fn from_db(address: Address, cryptor: &RingCryptor, key: &[u8]) -> AddressObject {
        AddressObject {
            id: address.id,
            title: String::from_utf8_lossy(&cryptor.open(key, &address.title).unwrap()).to_string(),
            label: String::from_utf8_lossy(&cryptor.open(key, &address.label).unwrap()).to_string(),
        }
    }
}

#[derive(FromRow)]
pub struct VehicleType {
    pub id: i16,
    pub label: String,
}

#[derive(SimpleObject)]
pub struct VehicleTypeObject {
    pub id: i16,
    pub label: String,
}

impl VehicleTypeObject {
    pub fn from_db(vehicle_type: VehicleType) -> VehicleTypeObject {
        VehicleTypeObject {
            id: vehicle_type.id,
            label: vehicle_type.label,
        }
    }
}

#[derive(FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub model: Vec<u8>,
    pub horsepower: i16,
    pub electric: bool,
    pub vehicle_type_id: i16,
    pub user_id: Uuid,
}

#[derive(SimpleObject)]
pub struct VehicleObject {
    pub id: Uuid,
    pub model: String,
    pub horsepower: i16,
    pub electric: bool,
    pub vehicle_type_id: i16,
}

impl VehicleObject {
    pub fn from_db(vehicle: Vehicle, cryptor: &RingCryptor, key: &[u8]) -> VehicleObject {
        VehicleObject {
            id: vehicle.id,
            model: String::from_utf8_lossy(&cryptor.open(key, &vehicle.model).unwrap()).to_string(),
            horsepower: vehicle.horsepower,
            electric: vehicle.electric,
            vehicle_type_id: vehicle.vehicle_type_id,
        }
    }
}

#[derive(FromRow)]
pub struct Person {
    pub id: Uuid,
    pub name: Vec<u8>,
    pub limit_distance: bool,
    pub user_id: Uuid,
}

#[derive(SimpleObject)]
pub struct PersonObject {
    pub id: Uuid,
    pub name: String,
    pub limit_distance: bool,
}

impl PersonObject {
    pub fn from_db(person: Person, cryptor: &RingCryptor, key: &[u8]) -> PersonObject {
        PersonObject {
            id: person.id,
            name: String::from_utf8_lossy(&cryptor.open(key, &person.name).unwrap()).to_string(),
            limit_distance: person.limit_distance,
        }
    }
}

#[derive(FromRow)]
pub struct Distance {
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub meters: i32,
}

#[derive(SimpleObject)]
pub struct DistanceObject {
    pub meters: i32,
}

impl DistanceObject {
    pub fn from_db(distance: Distance) -> DistanceObject {
        DistanceObject {
            meters: distance.meters,
        }
    }
}

#[derive(FromRow)]
pub struct Journey {
    pub id: Uuid,
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub person_id: Uuid,
    pub vehicle_id: Uuid,
    pub date: NaiveDate,
    pub meters: i32,
}

#[derive(SimpleObject)]
pub struct JourneyObject {
    pub from_id: Uuid,
    pub to_id: Uuid,
    pub person_id: Uuid,
    pub vehicle_id: Uuid,
    pub date: NaiveDate,
    pub meters: i32,
}

impl JourneyObject {
    pub fn from_db(journey: Journey) -> JourneyObject {
        JourneyObject {
            from_id: journey.from_id,
            to_id: journey.to_id,
            person_id: journey.person_id,
            vehicle_id: journey.vehicle_id,
            date: journey.date,
            meters: journey.meters,
        }
    }
}

#[derive(FromRow)]
pub struct Scale {
    pub id: Uuid,
    pub year: i16,
    pub horsepower_min: Option<i16>,
    pub horsepower_max: Option<i16>,
    pub first_threshold: i16,
    pub second_threshold: i16,
    pub first_slice_multiplier: BigDecimal,
    pub second_slice_multiplier: BigDecimal,
    pub third_slice_multiplier: BigDecimal,
    pub second_slice_fixed_amount: i16,
    pub vehicle_type_id: i16,
}

#[derive(SimpleObject)]
pub struct TotalObject {
    pub formula: String,
    pub total: i64,
}

#[derive(Serialize)]
pub struct AddressRequest {
    pub q: String,
}

#[derive(Deserialize)]
pub struct AddressProperties {
    pub label: String,
}

#[derive(Deserialize)]
pub struct AddressFeatures {
    pub properties: AddressProperties,
}

#[derive(Deserialize)]
pub struct AddressResponse {
    pub features: Vec<AddressFeatures>,
}
