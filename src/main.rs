mod crud;
mod routes;
mod utils;

use std::env::{self, set_var};

use axum::{
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use tower_http::services::ServeDir;
use utils::db::get_db;
use utils::ingest;

#[cfg(debug_assertions)]
fn load_env() {
    dotenv::dotenv().ok();
}

#[cfg(not(debug_assertions))]
fn load_env() {}

#[tokio::main]
async fn main() -> Result<(), Error> {
    load_env();
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let pool = get_db().await;

    println!("Starting migration");

    sqlx::migrate!()
        .set_locking(false)
        .run(&pool)
        .await
        .expect("Failed to perform database migrations");

    println!("Finished migration");

    if let Ok(env) = env::var("ENV") {
        if env == "PROD" {
            ingest::ingest_articles().await;
        }
    }

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::root::root))
        .route("/blogs", get(routes::blogs::blogs))
        .route("/blog/:id", get(routes::blog::blog))
        .route("/resume", get(routes::resume::resume))
        .route("/radar", get(routes::radar::radar))
        .route("/search", post(routes::search::search))
        .route("/health/", get(routes::health::health_check))
        .with_state(pool);

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    run(app).await
}
