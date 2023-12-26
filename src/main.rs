use axum::{routing::get, Router};
use tower_http::services::ServeDir;

mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::root::root))
        .route("/blogs", get(routes::blogs::blogs))
        .route("/blog/:id", get(routes::blog::blog));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
