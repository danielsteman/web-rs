use std::{env, fs};

use crate::{crud::blog::Blog, utils::db::get_db};
use regex::Regex;
use reqwest;
use serde_json::json;
use time::{macros::format_description, Date};

pub async fn ingest_articles() -> Option<()> {
    match fs::read_dir("./articles") {
        Ok(files) => {
            for file in files {
                let path = file.unwrap().path();
                let str_path = path.as_os_str();
                let content = fs::read_to_string(str_path)
                    .expect(format!("Error reading from {:?}", str_path).as_str());

                if let Some(metadata) = get_metadata(content.as_str()) {
                    println!("{:?}", metadata);
                    let blog = metadata_to_blog(metadata).await.unwrap();
                    println!("{:?}", blog);
                    let pool = get_db().await;

                    blog.create_blog(&pool).await.unwrap_or_else(|err| {
                        eprintln!("Error inserting blog: {}", err);
                    })
                } else {
                    eprintln!("No metadata found in file")
                }
            }
        }
        Err(e) => eprintln!("Error reading from dir `articles`: {}", e),
    }
    Some(())
}

async fn metadata_to_blog(metadata: Metadata) -> Option<Blog> {
    if metadata.is_complete() {
        let id = metadata.id.unwrap().parse::<i32>().ok()?;
        let title = metadata.title.clone().unwrap();
        let body = metadata.body.clone().unwrap();
        let summary = summarize(&body, generate_blog_summary_prompt)
            .await
            .unwrap();
        let string_date = metadata.date.clone().unwrap();
        let date_format = format_description!("[year]-[month]-[day]");
        let date = Date::parse(string_date.as_str(), &date_format).unwrap();

        let tags = metadata
            .tags
            .clone()
            .unwrap()
            .split(", ")
            .map(String::from)
            .collect();

        let blog = Blog {
            id,
            title,
            summary,
            body,
            date,
            tags,
        };

        Some(blog)
    } else {
        None
    }
}

#[derive(Debug)]
struct Metadata {
    id: Option<String>,
    title: Option<String>,
    body: Option<String>,
    date: Option<String>,
    tags: Option<String>,
}

impl Metadata {
    fn is_complete(&self) -> bool {
        self.id.is_some() && self.title.is_some() && self.date.is_some() && self.tags.is_some()
    }
}

fn get_metadata(text: &str) -> Option<Metadata> {
    let mut metadata = Metadata {
        id: None,
        title: None,
        body: None,
        date: None,
        tags: None,
    };

    let metadata_re = Regex::new(r"% (\w+): (.+)").unwrap();

    let text_as_lines: Vec<&str> = text.lines().collect();
    let body_lines = text_as_lines[4..].to_vec();
    metadata.body = Some(body_lines.join("\n"));

    for line in text.lines() {
        if let Some(capture) = metadata_re.captures(line) {
            if let (Some(key), Some(value)) = (capture.get(1), capture.get(2)) {
                match key.as_str() {
                    "id" => metadata.id = Some(value.as_str().to_string()),
                    "title" => metadata.title = Some(value.as_str().to_string()),
                    "date" => metadata.date = Some(value.as_str().to_string()),
                    "tags" => metadata.tags = Some(value.as_str().to_string()),
                    _ => {}
                }
            }
        }
    }

    if metadata.is_complete() {
        Some(metadata)
    } else {
        None
    }
}

async fn summarize(
    text: &str,
    prompt_generator: fn(&str) -> String,
) -> Result<String, reqwest::Error> {
    let api_url = "https://api.openai.com/v1/chat/completions";
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set.");

    let input_text = prompt_generator(text);
    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": input_text,
            "model": "gpt-3.5-turbo-1106"
        }))
        .send()
        .await?;

    println!("{:?}", response.text().await);

    Ok(String::from("hoi"))

    // match response.error_for_status() {
    //     Ok(res) => {
    //         let summary = res.json::<serde_json::Value>().await?;
    //         let summarized_text = summary["choices"][0]["text"].as_str().unwrap_or_default();

    //         println!("Summarized Text: {}", summarized_text);

    //         Ok(String::from(summarized_text))
    //     }
    //     Err(err) => {
    //         eprintln!("{}", err);
    //         panic!("Something went wrong during summarization using OpenAI")
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_test_prompt(text: &str) -> String {
        String::from(text)
    }

    #[tokio::test]
    async fn test_summarizer() {
        let prompt = "return only the string pass, and nothing else";
        let summary = summarize(prompt, generate_test_prompt).await.unwrap();
        assert_eq!(summary, "pass")
    }
}

fn generate_blog_summary_prompt(text: &str) -> String {
    let prompt = format!("This following piece of text is a blog post. What I would like is a three sentence summary of what the blog post is about. It should read as a preview and make the reader curious but the tone of voice should similar to the blog post itself: {}", text);
    prompt
}
