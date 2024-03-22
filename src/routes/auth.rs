use std::sync::{Arc, Mutex};

use axum::{http::Response, response::IntoResponse, Json};
use futures::lock::Mutex as AsyncMutex;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{
    auth::{documents::AllowedDocuments, generate_token, user::User},
    db::Database,
};

use super::responses::IndexResponse;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthenticatedResponse {
    token: Option<String>,
    success: bool,
}

pub async fn authenticate(
    state: Arc<AsyncMutex<Database>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let db = state.lock().await;
    let username = payload.username;

    let new_user = User {
        allowed_docs: AllowedDocuments::new(),
        password: "admin".to_string(),
        username: "admin".to_string(),
    };
    let user_created = db.create_user(new_user);
    println!("New User: {:#?}", user_created);

    match db.get_by_username(username.as_str()) {
        None => {
            return Response::new(
                to_string(&AuthenticatedResponse {
                    success: false,
                    token: None,
                })
                .unwrap(),
            );
        }
        Some(user) => {
            println!("AUTH {:#?}", user);
            match generate_token(username.as_str()) {
                Err(e) => {
                    return Response::new(
                        to_string(&AuthenticatedResponse {
                            success: false,
                            token: None,
                        })
                        .unwrap(),
                    );
                }
                Ok(token) => {
                    return Response::new(
                        to_string(&AuthenticatedResponse {
                            success: true,
                            token: Some(token),
                        })
                        .unwrap(),
                    );
                }
            };
        }
    };
}

pub async fn create_user(
    state: Arc<AsyncMutex<Database>>,
    Json(payload): Json<CreateUserPayload>,
) -> impl IntoResponse {
    let db = state.lock().await;
    let username = payload.username;
    let password = payload.password;

    let new_user = User {
        allowed_docs: AllowedDocuments::new(),
        password,
        username,
    };

    match db.create_user(new_user) {
        Err(sql_error) => {
            println!("Create User Error: {:#?}", sql_error);
            return Response::new(to_string(&IndexResponse { success: false }).unwrap());
        }
        Ok(resp) => {
            return Response::new(to_string(&IndexResponse { success: true }).unwrap());
        }
    };
}
