pub mod documents;
pub mod user;

use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::config::TOKEN_EXPIRE_TIME;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2,
};

use self::user::UserClaims;

pub fn generate_token(username: &str) -> Result<String, String> {
    let max_token_expire = TOKEN_EXPIRE_TIME.unwrap();
    let expiration = Utc::now() + max_token_expire;

    let claims = UserClaims {
        username: username.to_string(),
        expire: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| format!("Token generation error: {}", e))?;
    Ok(token)
}

pub fn hash_string(raw: &str) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(raw.as_bytes(), &salt).unwrap();

    match PasswordHash::new(&password_hash.to_string()) {
        Err(_) => return None,
        Ok(hash) => return Some(hash.to_string()),
    };
}
