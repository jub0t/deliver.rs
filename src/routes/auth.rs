use std::sync::{Arc, Mutex};

use axum::{extract::Path, response::IntoResponse, Json};
use serde::Deserialize;

use crate::{cache::Cache, db::Database};

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    password: String,
}

pub async fn authenticate(
    state: Arc<Mutex<Database>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let db = state.lock().unwrap();
}
