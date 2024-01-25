use askama::Template;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    employer: String,
    title: String,
    period: Period,
}

#[derive(Debug, Deserialize, Serialize)]
struct Study {
    school: String,
    title: String,
    period: Period,
}

#[derive(Debug, Deserialize, Serialize)]
struct Resume {
    experience: Vec<Experience>,
    education: Vec<Study>,
}

pub async fn resume() -> impl IntoResponse {
    let resume_data = json!({
        "experience": [
            {
                "employer": "Bridgefund",
                "title": "Data engineer",
                "period": {
                    "from": "October 2023",
                    "to": "Present"
                }
            },
            {
                "employer": "a.s.r.",
                "title": "Software engineer",
                "period": {
                    "from": "January 2021",
                    "to": "September 2023"
                }
            }
        ],
        "education": [
            {
                "school": "MOOC",
                "title": "Full Stack Open",
                "period": {
                    "from": "September 2020",
                    "to": "December 2020"
                }
            },
            {
                "school": "Vrije Universiteit Amsterdam",
                "title": "MSc Finance & Technology (honours programme)",
                "period": {
                    "from": "February 2019",
                    "to": "August 2020"
                }
            },
            {
                "school": "The Hague University of Applied Sciences",
                "title": "BSc International Business",
                "period": {
                    "from": "September 2013",
                    "to": "August 2017"
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
