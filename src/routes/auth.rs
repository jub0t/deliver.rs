use std::sync::{Arc, Mutex};

use axum::{response::IntoResponse, Json};
use serde::Deserialize;

use crate::{db::Database};

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    password: String,
}

pub async fn authenticate(
    state: Arc<Mutex<Database>>,
    Json(_payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let _db = state.lock().unwrap();
}
