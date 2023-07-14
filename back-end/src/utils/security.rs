use std::num::NonZeroU32;

use pbkdf2::password_hash::SaltString;
use rand_core::OsRng;
use ring::{
    digest::{self, SHA256},
    pbkdf2::{derive, PBKDF2_HMAC_SHA256},
};
use uuid::Uuid;

pub fn generate_password_salt(password: &str) -> (String, String) {
    let salt = SaltString::generate(&mut OsRng).to_string();
    let hashed_password = generate_password(password, salt.as_bytes());

    (salt.to_string(), hashed_password)
}

pub fn match_passwords(incoming_password: String, salt: String, user_password: String) -> bool {
    let hashed_password = generate_password(&incoming_password, salt.as_bytes());
    user_password == hashed_password
}

fn generate_password(password: &str, salt: &[u8]) -> String {
    let mut output = vec![0u8; 128]; // Output length
    let iterations: u32 = 122381;

    derive(
        PBKDF2_HMAC_SHA256,
        NonZeroU32::new(iterations).unwrap(),
        salt,
        password.as_bytes(),
        &mut output,
    );

    hex::encode(output)
}

pub fn generate_random_token() -> String {
    let uuid = Uuid::new_v4();
    let hashed = digest::digest(&SHA256, uuid.as_bytes());
    let hex_string = hex::encode(hashed.as_ref());

    hex_string
}
