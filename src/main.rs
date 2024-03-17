pub mod cache;
pub mod hasher;
pub mod minify;
pub mod watchdog;

use axum::{extract::Path, response::Response, routing::get, Router};
use cache::{load::load_into, Cache, File};
use serde::Serialize;
use serde_json::to_string;
use std::sync::{Arc, Mutex};

const STORE: &str = "./assets/";
static MAX_CACHE_TIME: u64 = 60;

pub type CacheArc = Arc<Mutex<Cache>>;

#[derive(Serialize)]
pub struct IndexResponse {
    pub bytes_cached: usize,
    data: Vec<File>,
}

#[tokio::main]
async fn main() {
    let mut cache = Cache::new();
    load_into(&mut cache);

    let shared_cache = Arc::new(Mutex::new(cache));
    println!("Cache Successful!");

    let shared_cache_clone = Arc::clone(&shared_cache);
    let app = Router::new()
        .route(
            "/:document/:id",
            get(move |path| get_asset(shared_cache_clone.clone(), path)),
        )
        .route("/all", get(move || get_all_assets(shared_cache.clone())));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3434")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_asset(
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

async fn get_all_assets(state: Arc<Mutex<Cache>>) -> Response {
    let cache = state.lock().unwrap();
    let files = cache.as_vec();

    let r = IndexResponse {
        bytes_cached: cache.size(),
        data: files,
    };

    let data = to_string(&r).unwrap();
    Response::new(data.into())
}
