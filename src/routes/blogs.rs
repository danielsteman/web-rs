use askama::Template;
use axum::{extract::State, response::IntoResponse};
use sqlx::PgPool;

use crate::utils::html::HtmlTemplate;

#[derive(Template)]
#[template(path = "blogs.html")]
struct BlogsTemplate {
    blogs: Vec<Blog>,
}

struct Blog {
    id: i16,
    title: String,
    summary: String,
}

pub async fn blogs(State(pool): State<PgPool>) -> impl IntoResponse {
    let template = BlogsTemplate {
        blogs: vec![
            Blog {
                id: 1,
                title: "Integrating a ML model in an API 🔀".to_string(),
                summary: "For a project I was working on, we needed more than just the service, we also needed to store predictions and apply some business logic.".to_string()
            },
            Blog {
                id: 2,
                title: "WASM with Javascript and Rust 🦀".to_string(),
                summary: "Web Assembly (WASM) is a new approach towards web development, which leverages the speed and robustness of lower level languages such as C, C++ and Rust to power websites. In this post I will go through an example in Rust.".to_string()
            },
        ],
    };
    HtmlTemplate(template)
}
