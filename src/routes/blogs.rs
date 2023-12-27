use askama::Template;
use axum::{extract::State, response::IntoResponse};
use sqlx::PgPool;

use crate::crud::blog::Blog;
use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsTemplate {
    blogs: Vec<Blog>,
}

pub async fn blogs(State(pool): State<PgPool>) -> impl IntoResponse {
    match Blog::get_blogs(&pool).await {
        Ok(blogs) => {
            let template = BlogsTemplate { blogs };
            HtmlTemplate(template)
        }
        Err(err) => {
            println!("Error fetching blogs: {}", err);

            let error_template = BlogsTemplate { blogs: vec![] };
            HtmlTemplate(error_template)
        }
    }
}
