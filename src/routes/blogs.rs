use askama::Template;
use axum::{extract::State, response::IntoResponse};
use sqlx::PgPool;

use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsTemplate {
    blogs: Vec<Blog>,
}

#[derive(Debug, sqlx::FromRow)]
struct Blog {
    id: i32,
    title: String,
    summary: String,
}

pub async fn blogs(State(pool): State<PgPool>) -> impl IntoResponse {
    let blogs: Vec<Blog> = sqlx::query_as::<_, Blog>("SELECT id, title, summary FROM blog")
        .fetch_all(&pool)
        .await
        .unwrap();

    for blog in &blogs {
        println!("{:?}", blog.id);
    }

    let template = BlogsTemplate { blogs };
    HtmlTemplate(template)
}
