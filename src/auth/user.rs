use serde::{Deserialize, Serialize};

use super::documents::AllowedDocuments;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub allowed_docs: AllowedDocuments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub exp: usize,
}

impl UserClaims {
    pub fn new(exp: usize) -> Self {
        Self { exp }
    }
}
