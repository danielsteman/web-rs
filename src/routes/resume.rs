use askama::Template;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::utils::html::HtmlTemplate;

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
struct School {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Experience {
    employer: String,
    title: String,
    period: Period,
}

#[derive(Debug, Deserialize, Serialize)]
struct Study {
    school: School,
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
                "title": "Soft Engineer - Data Platform",
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
            },
            {
                "employer": "Virtuagym",
                "title": "Analyst",
                "period": {
                    "from": "August 2017",
                    "to": "December 2020"
                }
            }
        ],
        "education": [
            {
                "school": {
                    "name": "MOOC",
                    "url": "https://fullstackopen.com/en/"
                },
                "title": "Full Stack Open",
                "period": {
                    "from": "September 2020",
                    "to": "December 2020"
                }
            },
            {
                "school": {
                    "name": "Vrije Universiteit Amsterdam",
                    "url": "https://vu.nl/en/education/master/finance-duisenberg-honours-programme-in-finance-and-technology"
                },
                "title": "MSc Finance & Technology (honours programme)",
                "period": {
                    "from": "February 2019",
                    "to": "August 2020"
                }
            },
            {
                "school": {
                    "name": "The Hague University of Applied Sciences",
                    "url": "https://www.dehaagsehogeschool.nl/opleidingen/hbo-bachelor/international-business-4-jaar"
                },
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
