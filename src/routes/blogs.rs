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
    page: usize,
    per_page: usize,
    total_pages: usize,
    prev_page: Option<usize>,
    next_page: Option<usize>,
}

fn default_page() -> usize {
    1
}

fn default_per_page() -> usize {
    10
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[serde(default = "default_page")]
    page: usize,
    #[serde(default = "default_per_page")]
    per_page: usize,
}

pub async fn blogs(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let pagination = match pagination {
        Some(Query(pagination)) => pagination,
        None => Pagination {
            page: default_page(),
            per_page: default_per_page(),
        },
    };

    let per_page = pagination.per_page.clamp(1, 100);
    let total = Blog::count_blogs(&pool).await.unwrap_or(0).max(0) as usize;
    let total_pages = ((total + per_page - 1) / per_page).max(1);
    let page = pagination.page.clamp(1, total_pages);
    let offset = (page - 1) * per_page;

    let blogs = match Blog::get_blogs(&pool, per_page, offset).await {
        Ok(blogs) => blogs,
        Err(err) => {
            eprintln!("Error fetching blogs: {}", err);
            vec![]
        }
    };

    let template = BlogsTemplate {
        blogs,
        page,
        per_page,
        total_pages,
        prev_page: (page > 1).then(|| page - 1),
        next_page: (page < total_pages).then(|| page + 1),
    };

    HtmlTemplate(template)
}
