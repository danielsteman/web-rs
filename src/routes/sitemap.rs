use axum::{
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};

pub async fn sitemap_xml() -> impl IntoResponse {
    let content = include_str!("../../sitemap.xml");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "application/xml; charset=utf-8".parse().unwrap(),
    );
    (StatusCode::OK, headers, content)
}
