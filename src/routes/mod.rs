pub mod auth;
pub mod middleware;
pub mod responses;
pub mod stream;

use std::{
    fs,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Path, Request},
    response::{IntoResponse, Response},
};
use http::header::CACHE_CONTROL;
use serde_json::to_string;
use uuid::Uuid;

use crate::{
    cache::{format::format_to_mime, Cache, CacheOptions},
    config::STORE,
};

use self::responses::{
    CustomDataResponse, DiagnosticsResponse, IndexResponse, ListAllResponse, MessageResponse,
};

pub async fn create_document(_req: Request) -> impl IntoResponse {
    let path = Uuid::new_v4();

    match fs::create_dir(format!("{}/{}", STORE, path)) {
        Ok(_) => Response::new(
            to_string(&CustomDataResponse {
                message: "Document Created, Success".to_string(),
                success: true,
                data: path.to_string(),
            })
            .unwrap(),
        ),
        Err(error) => Response::new(
            to_string(&MessageResponse {
                message: format!("Internal Server Errro - {:#?}", error),
                success: false,
            })
            .unwrap(),
        ),
    }
}

pub async fn upload_content() {}

pub async fn other_routes() -> Response<String> {
    Response::new(to_string(&IndexResponse { success: true }).unwrap())
}

pub async fn get_asset(
    state: Arc<Mutex<Cache>>,
    Path((docid, assid)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut cache = state.lock().unwrap();
    let file = cache.get(docid.clone(), assid.clone());

    match file {
        None => {
            // File is not found in cache.
            // Let's re-cache the file and send back.
            cache.cache(docid, assid, CacheOptions { minify: true });

            (
                [
                    ("content-type", "text/plain".to_string()),
                    (("cache-control", "public, max-age=3600".to_string())),
                ],
                Vec::new(),
            )
        }
        Some(file) => {
            let contents = file.contents.clone(); // The cached contents
            let content_type = format_to_mime(file.format);

            (
                [
                    ("content-type", content_type),
                    (("cache-control", "public, max-age=3600".to_string())),
                ],
                contents,
            )
        }
    }
}

pub async fn diagnostics(state: Arc<Mutex<Cache>>) -> Response<String> {
    let cache = state.lock().unwrap();

    let r = DiagnosticsResponse {
        bytes_cached: cache.size(),
        total_files: cache.item_count(),
    };

    let data = to_string(&r).unwrap();
    Response::new(data)
}

pub async fn list_assets(state: Arc<Mutex<Cache>>) -> Response<String> {
    let cache = state.lock().unwrap();
    let files = cache.as_contentless_vec();

    let r = ListAllResponse { files };
    let data = to_string(&r).unwrap();

    Response::new(data)
}
