use core::panic;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use sqlx::PgPool;

use crate::{crud::blog::Blog, utils::html::HtmlTemplate};

use askama::Template;
use sqlx::types::time::Date;

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogTemplate {
    id: i32,
    title: String,
    summary: String,
    body: String,
    date: Date,
    tags: Vec<String>,
}

pub async fn blog(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match Blog::get_blog(&pool, id).await {
        Ok(blog) => HtmlTemplate(BlogTemplate {
            id: blog.id,
            title: blog.title.clone(),
            summary: blog.summary.clone(),
            body: blog.body.clone(),
            date: blog.date,
            tags: blog.tags.clone(),
        }),
        Err(err) => {
            println!("Error fetching blog: {}", err);
            panic!("Error fetching blog")
        }
    }
}
