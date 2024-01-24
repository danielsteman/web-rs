use askama::Template;
use axum::response::IntoResponse;

use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "radar.html")]
struct RadarTemplate {}

pub async fn radar() -> impl IntoResponse {
    let template = RadarTemplate {};
    HtmlTemplate(template)
}
