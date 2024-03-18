pub mod cache;
pub mod hasher;
pub mod minify;
pub mod routes;
pub mod watchdog;

use axum::{
    routing::{get, post},
    Router,
};
use cache::{load::load_into, Cache};
use colored::Colorize;
use std::{
    sync::{Arc, Mutex},
    thread,
};

const STORE: &str = "./test/";

pub type CacheArc = Arc<Mutex<Cache>>;

#[tokio::main]
async fn main() {
    let mut cache = Cache::new();
    load_into(&mut cache);

    let shared_cache = Arc::new(Mutex::new(cache));

    let watchdog_cache = Arc::clone(&shared_cache);
    thread::spawn(move || {
        watchdog::start(watchdog_cache);
    });

    let cache1 = Arc::clone(&shared_cache.clone());

    let app = Router::new()
        .route(
            "/:document/:id",
            get(move |path| routes::get_asset(cache1, path)),
        )
        .route("/all", get(move || routes::get_all_assets(shared_cache)))
        .route("/create-document", post(routes::create_document))
        .route("/upload-content", post(routes::upload_content))
        .route("/", get(routes::other_routes));

    println!("{} Running at http://127.0.0.1:3434", "[SERVER]".red());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3434")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
