use axum::{routing::get, Router};
mod routes;
mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::root::root))
        .route("/blog/:id", get(routes::blog::blog));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
