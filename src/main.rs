mod crud;
mod routes;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use utils::db::get_db;
// use utils::ingest;

#[cfg(debug_assertions)]
fn load_env() {
    dotenv::dotenv().ok();
}

#[cfg(not(debug_assertions))]
fn load_env() {}

#[tokio::main]
async fn main() {
    load_env();

    let pool = get_db().await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to perform database migrations");

    // ingest::ingest_articles().await;

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::root::root))
        .route("/blogs", get(routes::blogs::blogs))
        .route("/blog/:id", get(routes::blog::blog))
        .route("/resume", get(routes::resume::resume))
        .route("/search", post(routes::search::search))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
