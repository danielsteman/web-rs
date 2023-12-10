use super::html::HtmlTemplate;
use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct RootTemplate {}

pub async fn root() -> impl IntoResponse {
    let template = RootTemplate {};
    HtmlTemplate(template)
}
