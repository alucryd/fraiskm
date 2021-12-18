use crate::database::*;
use ring::pbkdf2;
use sqlx::postgres::PgConnection;
use tindercrypt::cryptors::RingCryptor;
use tindercrypt::errors::Error;
use tindercrypt::pbkdf2::derive_key;
use uuid::Uuid;

pub fn hash_password(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_hex().to_string()
}

pub fn derive_pbkdf2_key(username: &str, password: &str) -> String {
    let digest_algo = pbkdf2::PBKDF2_HMAC_SHA256;
    let iterations = 100000;
    let mut key = [0u8; 32];
    derive_key(
        digest_algo,
        iterations,
        env!("TINDERCRYPT_SALT").as_bytes(),
        vec![username, password].join(":").as_bytes(),
        &mut key,
    )
    .unwrap();
    base64::encode(&key)
}

pub fn decode_key(key: &str) -> Vec<u8> {
    base64::decode(key).unwrap()
}

pub fn encrypt_data(cryptor: &RingCryptor, key: &str, data: &str) -> Result<Vec<u8>, Error> {
    Ok(cryptor.seal_with_key(&decode_key(&key), data.as_bytes())?)
}

pub fn decrypt_data(cryptor: &RingCryptor, key: &str, data: &[u8]) -> Result<String, Error> {
    Ok(String::from_utf8_lossy(&cryptor.open(&decode_key(key), data)?).to_string())
}

pub fn reencrypt_data(
    cryptor: &RingCryptor,
    key: &str,
    new_key: &str,
    data: &[u8],
) -> Result<Vec<u8>, Error> {
    Ok(encrypt_data(
        cryptor,
        new_key,
        &decrypt_data(cryptor, key, data)?,
    )?)
}

pub async fn reencrypt_all_data(
    transaction: &mut PgConnection,
    cryptor: &RingCryptor<'_>,
    key: &str,
    id: &Uuid,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    let new_key = derive_pbkdf2_key(username, password);
    let addresses = find_addresses_by_user_id(transaction, id).await;
    let drivers = find_drivers_by_user_id(transaction, id).await;
    let vehicles = find_vehicles_by_user_id(transaction, id).await;
    for address in addresses {
        update_address(
            transaction,
            &address.id,
            &reencrypt_data(cryptor, &key, &new_key, &address.title)?,
            &reencrypt_data(cryptor, &key, &new_key, &address.label)?,
            address.address_type,
        )
        .await;
    }
    for driver in drivers {
        update_driver(
            transaction,
            &driver.id,
            &reencrypt_data(cryptor, &key, &new_key, &driver.name)?,
            driver.limit_distance,
            driver.default_vehicle_id.as_ref(),
            driver.default_from_id.as_ref(),
            driver.default_to_id.as_ref(),
        )
        .await;
    }
    for vehicle in vehicles {
        update_vehicle(
            transaction,
            &vehicle.id,
            &reencrypt_data(cryptor, &key, &new_key, &vehicle.model)?,
            vehicle.horsepower,
            vehicle.electric,
            vehicle.vehicle_type,
        )
        .await;
    }
    Ok(())
}
