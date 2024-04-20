use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use serde_json::to_string;

use crate::{auth::validate_token, routes::responses::MessageResponse};

pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let token = headers.get("token");

    match token {
        None => Ok(Response::new(
            to_string(&MessageResponse {
                message: "Token not found in headers".to_string(),
                success: false,
            })
            .unwrap()
            .into(),
        )),
        Some(token) => {
            let raw = token.to_str().unwrap();

            if validate_token(raw) {
                let response = next.run(req).await;
                Ok(response)
            } else {
                Ok(Response::new(
                    to_string(&MessageResponse {
                        message: "Invalid JsonWebToken".to_string(),
                        success: true,
                    })
                    .unwrap()
                    .into(),
                ))
            }
        }
    }
}
