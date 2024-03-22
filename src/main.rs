pub mod auth;
pub mod cache;
pub mod config;
pub mod db;
pub mod hasher;
pub mod minify;
pub mod routes;
pub mod watchdog;

use axum::extract::{Extension, State};
use axum::{
    routing::{get, post},
    Router,
};
use cache::{load::load_into, Cache};
use colored::Colorize;
use futures::lock::Mutex as AsyncMutex;
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[tokio::main]
async fn main() {
    let mut cache = Cache::new();
    load_into(&mut cache);

    let shared_cache = Arc::new(Mutex::new(cache));

    let db = db::Database::new();
    let db_arc = Arc::new(AsyncMutex::new(db));
    let db_arc_cloned = Arc::clone(&db_arc);

    let watchdog_cache = Arc::clone(&shared_cache);
    thread::spawn(move || {
        watchdog::start(watchdog_cache);
    });

    let cache1 = Arc::clone(&shared_cache.clone());
    let cache_all = Arc::clone(&shared_cache.clone());

    let app = Router::new()
        .route(
            "/:document/:id",
            get(move |path| routes::get_asset(cache1, path)),
        )
        .route(
            "/diagnostics",
            get(move || routes::diagnostics(shared_cache)),
        )
        .route("/list-assets", get(move || routes::list_assets(cache_all)))
        .route("/create-document", post(routes::create_document))
        .route("/upload-content", post(routes::upload_content))
        .route(
            "/authenticate",
            post(move |body| routes::auth::authenticate(db_arc, body)),
        )
        .route(
            "/create-user",
            post(move |body| routes::auth::create_user(db_arc_cloned, body)),
        )
        .route("/", get(routes::other_routes));

    println!("{} Running at http://127.0.0.1:3434", "[SERVER]".red());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3434")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
