use askama::Template;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::utils::html::HtmlTemplate;

// TODO: serialize json into Rust types that implement Display

#[derive(Template)]
#[template(path = "resume.html")]
struct ResumeTemplate {
    resume_data: Resume,
}

#[derive(Debug, Deserialize, Serialize)]
struct Period {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Experience {
    title: String,
    period: Period,
}

#[derive(Debug, Deserialize, Serialize)]
struct Resume {
    experience: Vec<Experience>,
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
    });

    let deser_resume_data: Resume = serde_json::from_value(resume_data).unwrap();

    let template = ResumeTemplate {
        resume_data: deser_resume_data,
    };
    HtmlTemplate(template)
}
