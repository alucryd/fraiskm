use async_graphql::SimpleObject;
use sqlx::FromRow;
use tindercrypt::cryptors::RingCryptor;
use uuid::Uuid;

#[derive(FromRow)]
pub struct RowId {
    pub id: Uuid,
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
pub struct Car {
    pub id: Uuid,
    pub model: Vec<u8>,
    pub horsepower: i16,
    pub user_id: Uuid,
}

#[derive(SimpleObject)]
pub struct CarObject {
    pub id: Uuid,
    pub model: String,
    pub horsepower: i16,
}

impl CarObject {
    pub fn from_db(car: Car, cryptor: &RingCryptor, key: &[u8]) -> CarObject {
        CarObject {
            id: car.id,
            model: String::from_utf8_lossy(&cryptor.open(key, &car.model).unwrap()).to_string(),
            horsepower: car.horsepower,
        }
    }
}
#[derive(FromRow)]
pub struct Person {
    pub id: Uuid,
    pub name: Vec<u8>,
    pub user_id: Uuid,
}

#[derive(SimpleObject)]
pub struct PersonObject {
    pub id: Uuid,
    pub name: String,
}

impl PersonObject {
    pub fn from_db(person: Person, cryptor: &RingCryptor, key: &[u8]) -> PersonObject {
        PersonObject {
            id: person.id,
            name: String::from_utf8_lossy(&cryptor.open(key, &person.name).unwrap()).to_string(),
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
