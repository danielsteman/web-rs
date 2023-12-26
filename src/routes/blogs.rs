use askama::Template;
use axum::response::IntoResponse;

use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsTemplate {}

pub async fn blogs() -> impl IntoResponse {
    let template = BlogsTemplate {};
    HtmlTemplate(template)
}
