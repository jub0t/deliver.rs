pub mod auth;
pub mod cache;
pub mod config;
pub mod db;
pub mod hasher;
pub mod imgman;
pub mod midware;
pub mod minify;
pub mod routes;
pub mod watchdog;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use cache::Cache;
use colored::Colorize;
use futures::lock::Mutex as AsyncMutex;
use http::Method;
use std::{
    sync::{Arc, Mutex},
    thread,
};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cache = Cache::new();
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

    let cors = CorsLayer::new().allow_methods(vec![Method::GET, Method::POST]);

    let app = Router::new()
        .route("/upload-content", post(routes::upload_content))
        .route("/create-document", post(routes::create_document))
        .route_layer(middleware::from_fn(routes::middleware::auth_middleware))
        .route(
            "/:document/:id",
            get(move |path| routes::get_asset(cache1, path)),
        )
        .route(
            "/diagnostics",
            get(move || routes::diagnostics(shared_cache)),
        )
        .route("/list-assets", get(move || routes::list_assets(cache_all)))
        .route(
            "/authenticate",
            post(move |body| routes::auth::authenticate(db_arc, body)),
        )
        .route(
            "/create-user",
            post(move |body| routes::auth::create_user(db_arc_cloned, body)),
        )
        .route("/", get(routes::other_routes))
        .layer(cors);

    println!(
        "{} REST API Running at http://127.0.0.1:3434",
        "[SERVER]".red()
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3434")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
