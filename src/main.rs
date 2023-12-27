use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::services::ServeDir;

mod routes;
mod utils;

#[cfg(debug_assertions)]
fn load_env() {
    dotenv::dotenv().ok();
}

#[cfg(not(debug_assertions))]
fn load_env() {}

#[tokio::main]
async fn main() {
    load_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to perform database migrations");

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::root::root))
        .route("/blogs", get(routes::blogs::blogs))
        .route("/blog/:id", get(routes::blog::blog))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
