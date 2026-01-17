use time::macros::format_description;
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
    summary: String,
    body: String,
    date: Date,
    date_iso: String,
    keywords: String,
    tags_keywords: String,
    has_tags: bool,
}

pub async fn blog(State(pool): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    match Blog::get_blog(&pool, id).await {
        Ok(blog) => {
            // Format date as ISO 8601 for structured data
            let date_format = format_description!("[year]-[month]-[day]");
            let date_iso = blog.date.format(&date_format).unwrap_or_default();

            // Create keywords string from tags
            let keywords = if blog.tags.is_empty() {
                "Daniel Steman, software engineering, tech blog".to_string()
            } else {
                format!(
                    "{}, Daniel Steman, software engineering, tech blog",
                    blog.tags.join(", ")
                )
            };

            // Create tags keywords for structured data
            let tags_keywords = blog.tags.join(", ");

            HtmlTemplate(BlogTemplate {
                id,
                title: blog.title.clone(),
                summary: blog.summary.clone(),
                body: blog.body.clone(),
                date: blog.date,
                date_iso,
                keywords,
                tags_keywords,
                has_tags: !blog.tags.is_empty(),
            })
        }
        Err(_) => {
            let error_date = Date::from_calendar_date(1995, Month::April, 13).unwrap();
            let date_format = format_description!("[year]-[month]-[day]");
            let date_iso = error_date.format(&date_format).unwrap_or_default();

            HtmlTemplate(BlogTemplate {
                id: 0,
                title: "Not Found".to_string(),
                summary: "Blog post not found".to_string(),
                body: "This blog hasn't been written yet...".to_string(),
                date: error_date,
                date_iso,
                keywords: "Daniel Steman, software engineering, tech blog".to_string(),
                tags_keywords: String::new(),
                has_tags: false,
            })
        }
    }
}
