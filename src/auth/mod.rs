pub mod documents;
pub mod user;

use chrono::{Utc};
use colored::Colorize;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

use crate::config::TOKEN_EXPIRE_TIME;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};

use self::user::UserClaims;

const SECRET_KEY: &[u8] = b"secret";

pub fn generate_token(_username: &str) -> Result<String, String> {
    let max_token_expire = TOKEN_EXPIRE_TIME.unwrap(); // Default token expiration to 1 day
    let expiration = Utc::now() + max_token_expire;

    let claims = UserClaims::new(expiration.timestamp_micros() as usize);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
    .map_err(|e| format!("Token generation error: {}", e))
}

pub fn validate_token(token: &str) -> bool {
    match decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    ) {
        Err(error) => {
            println!("{} JsonWebToken Error: {:#?}", "[AUTH]:".cyan(), error);
            false
        }

        Ok(TokenData { claims, .. }) => {
            let now = Utc::now().timestamp() as usize;
            claims.exp > now
        }
    }
}

pub fn hash_string(raw: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(raw.as_bytes(), &salt).unwrap();

    match PasswordHash::new(&password_hash.to_string()) {
        Err(_) => None,
        Ok(hash) => Some(hash.to_string()),
    }
}
