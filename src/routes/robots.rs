use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

pub async fn robots_txt() -> impl IntoResponse {
    let content = include_str!("../../robots.txt");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    (StatusCode::OK, headers, content)
}
