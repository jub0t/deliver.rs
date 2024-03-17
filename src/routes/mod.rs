use std::sync::{Arc, Mutex};

use axum::{extract::Path, response::Response};
use serde::Serialize;
use serde_json::to_string;

use crate::cache::{Cache, File};

#[derive(Serialize)]
pub struct IndexResponse {
    pub bytes_cached: usize,
    data: Vec<File>,
}

pub async fn create_document() {}
pub async fn upload_content() {}
pub async fn other_routes() -> Response {
    return Response::new("success".into());
}

pub async fn get_asset(
    state: Arc<Mutex<Cache>>,
    Path((docid, assid)): Path<(String, String)>,
) -> Response {
    let cache = state.lock().unwrap();
    let file = cache.get(docid.clone(), assid.clone());

    match file {
        None => Response::new("file error".to_string().into()),
        Some(file) => {
            let contents = file.contents.clone();
            Response::new(contents.into())
        }
    }
}

pub async fn get_all_assets(state: Arc<Mutex<Cache>>) -> Response {
    let cache = state.lock().unwrap();
    let files = cache.as_vec();

    let r = IndexResponse {
        bytes_cached: cache.size(),
        data: files,
    };

    let data = to_string(&r).unwrap();
    Response::new(data.into())
}
