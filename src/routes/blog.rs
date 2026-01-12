use time::Month;

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
    body: String,
    date: Date,
}

pub async fn blog(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match Blog::get_blog(&pool, id).await {
        Ok(blog) => HtmlTemplate(BlogTemplate {
            id,
            title: blog.title.clone(),
            body: blog.body.clone(),
            date: blog.date,
        }),
        Err(_) => HtmlTemplate(BlogTemplate {
            id: 0,
            title: "Not Found".to_string(),
            body: "This blog hasn't been written yet...".to_string(),
            date: Date::from_calendar_date(1995, Month::April, 13).unwrap(),
        }),
    }
}
