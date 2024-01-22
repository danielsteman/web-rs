use serde_json::json;
use std::fs;

pub async fn json_parser(path: &str) {
    let json_resume = json!({
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
    // let json_data = serde_json::Deserializer();
    let parsed_json = json_resume.as_object().unwrap();
    println!("{:?}", parsed_json.get("experience").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_json_parser() {
        let data = json_parser("assets/resume.json").await;
        assert_eq!(3, 4)
    }
}
