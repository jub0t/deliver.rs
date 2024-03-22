use serde::{Deserialize, Serialize};

use super::documents::AllowedDocuments;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub allowed_docs: AllowedDocuments,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub username: String,
    pub expire: usize,
}
