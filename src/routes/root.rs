use askama::Template;
use axum::response::IntoResponse;

use crate::templates::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {}

pub async fn root() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}
