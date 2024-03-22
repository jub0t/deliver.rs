use std::sync::Arc;

use axum::{http::Response, response::IntoResponse, Json};
use colored::Colorize;
use futures::lock::Mutex as AsyncMutex;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{
    auth::{documents::AllowedDocuments, generate_token, user::User},
    db::Database,
};

use super::responses::MessageResponse;

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

    match db.get_by_username(username.as_str()) {
        None => Response::new(
            to_string(&AuthenticatedResponse {
                success: false,
                token: None,
            })
            .unwrap(),
        ),
        Some(user) => {
            println!("{} Authenticating: {:#?}", "[AUTH]:".cyan(), user.username);
            match generate_token(username.as_str()) {
                Err(_e) => Response::new(
                    to_string(&AuthenticatedResponse {
                        success: false,
                        token: None,
                    })
                    .unwrap(),
                ),
                Ok(token) => Response::new(
                    to_string(&AuthenticatedResponse {
                        success: true,
                        token: Some(token),
                    })
                    .unwrap(),
                ),
            }
        }
    }
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
        Err(sql_error) => match sql_error {
            rusqlite::Error::QueryReturnedNoRows => {
                return Response::new(
                    to_string(&MessageResponse {
                        success: false,
                        message: "User already exists".to_string(),
                    })
                    .unwrap(),
                );
            }

            _ => {
                return Response::new(
                    to_string(&MessageResponse {
                        success: false,
                        message: sql_error.to_string(),
                    })
                    .unwrap(),
                );
            }
        },
        Ok(_) => Response::new(
            to_string(&MessageResponse {
                success: true,
                message: "".to_string(),
            })
            .unwrap(),
        ),
    }
}
