use askama::Template;
use axum::response::IntoResponse;
use serde_json::{json, Map, Value};

use crate::utils::html::HtmlTemplate;

// TODO: serialize json into Rust types that implement Display

#[derive(Template)]
#[template(path = "resume.html")]
struct ResumeTemplate {
    resume_data: Map<String, Value>,
}

pub async fn resume() -> impl IntoResponse {
    let resume_data = json!({
        "experience": [
            {
                "title": "data engineer",
                "period": {
                    "from": "october 2023",
                    "to": "present"
                }
            },
            {
                "title": "software engineer",
                "period": {
                    "from": "january 2021",
                    "to": "september 2023"
                }
            }
        ]
    })
    .as_object()
    .unwrap()
    .to_owned();

    let template = ResumeTemplate { resume_data };
    HtmlTemplate(template)
}
