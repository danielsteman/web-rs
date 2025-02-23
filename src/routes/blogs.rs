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
    pagination: Pagination,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: usize,
    per_page: usize,
}

pub async fn blogs(
    State(pool): State<PgPool>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let pagination = match pagination {
        Some(pagination) => Pagination { ..*pagination },
        None => Pagination {
            page: 1,
            per_page: 20,
        },
    };

    let limit = pagination.per_page;
    let offset = (pagination.page - 1) * limit;

    match Blog::get_blogs(&pool, limit, offset).await {
        Ok(blogs) => {
            let pagination_data = Pagination {
                page: pagination.page,
                per_page: pagination.per_page,
            };
            let template = BlogsTemplate {
                blogs,
                pagination: pagination_data,
            };
            HtmlTemplate(template)
        }
        Err(err) => {
            eprintln!("Error fetching blogs: {}", err);

            let error_template = BlogsTemplate {
                blogs: vec![],
                pagination: Pagination {
                    page: 0,
                    per_page: 0,
                },
            };
            HtmlTemplate(error_template)
        }
    }
}
