use ring::pbkdf2;
use tindercrypt::pbkdf2::derive_key;

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

pub fn hash_password(password: &str) -> String {
    blake3::hash(password.as_bytes()).to_hex().to_string()
}
