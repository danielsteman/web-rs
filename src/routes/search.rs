use askama::Template;
use axum::extract::Query;
use axum::{extract::State, response::IntoResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::crud::blog::Blog;
use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsTemplate {
    blogs: Vec<Blog>,
}

#[derive(Deserialize)]
pub struct Search {
    search_string: String,
}

pub async fn search(State(pool): State<PgPool>, Query(params): Query<Search>) -> impl IntoResponse {
    match Blog::search_blogs(&pool, params.search_string.as_str()).await {
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
