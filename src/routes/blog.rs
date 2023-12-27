use askama::Template;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    title: String,
    body: String,
}

pub async fn blog(State(pool): State<PgPool>, Path(id): Path<u32>) -> impl IntoResponse {
    let template = BlogTemplate {
        title: String::from("Hoi"),
        body: id.to_string(),
    };
    HtmlTemplate(template)
}
