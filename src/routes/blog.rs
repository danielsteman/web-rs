use core::panic;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

use sqlx::PgPool;

use crate::{crud::blog::Blog, utils::html::HtmlTemplate};

pub async fn blog(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match Blog::get_blog(&pool, id).await {
        Ok(blog) => HtmlTemplate(blog),
        Err(err) => {
            println!("Error fetching blog: {}", err);
            panic!("Error fetching blog")
        }
    }
}
