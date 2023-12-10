use askama::Template;
use axum::{extract::Path, response::IntoResponse};

use crate::templates::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    body: String,
}

pub async fn blog(Path(id): Path<u32>) -> impl IntoResponse {
    let template = BlogTemplate {
        title: String::from("Hoi"),
        body: id.to_string(),
    };
    HtmlTemplate(template)
}
