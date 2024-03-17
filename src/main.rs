pub mod cache;
pub mod hasher;
pub mod minify;
pub mod routes;
pub mod watchdog;

use axum::{routing::get, Router};
use cache::{load::load_into, Cache};
use std::sync::{Arc, Mutex};

const STORE: &str = "./test/";

pub type CacheArc = Arc<Mutex<Cache>>;

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
            get(move |path| routes::get_asset(shared_cache_clone.clone(), path)),
        )
        .route(
            "/all",
            get(move || routes::get_all_assets(shared_cache.clone())),
        )
        .route("/create-document", get(routes::create_document))
        .route("/upload-content", get(routes::upload_content))
        .route("/", get(routes::other_routes));

    println!("Running at http://127.0.0.1:3434");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3434")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}