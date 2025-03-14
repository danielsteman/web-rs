use std::fs;

use crate::{crud::blog::Blog, utils::db::get_db};
use regex::Regex;
use time::{macros::format_description, Date};

pub async fn ingest_articles() -> Option<()> {
    match fs::read_dir("./articles") {
        Ok(files) => {
            for file in files {
                let path = file.unwrap().path();

                let content = fs::read_to_string(&path)
                    .expect(format!("Error reading from {:?}", &path).as_str());

                if let Some(blog_id) = get_id(content.as_str()).await {
                    if !blog_exists(&blog_id).await {
                        if let Some(metadata) = get_metadata(content.as_str()) {
                            let blog = metadata_to_blog(metadata).await.unwrap();
                            let pool = get_db().await;

                            blog.create_blog(&pool).await.unwrap_or_else(|err| {
                                eprintln!("Error inserting blog: {}", err);
                            });
                            println!("Added blog {}", blog.id);
                        } else {
                            eprintln!("No metadata found, skipping blog {}", blog_id)
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("Error reading from dir `articles`: {}", e),
    }
    Some(())
}

async fn get_id(text: &str) -> Option<i32> {
    let id_re = Regex::new(r"% id: (.+)").unwrap();
    for line in text.lines() {
        if let Some(capture) = id_re.captures(line) {
            if let Some(id) = capture.get(1) {
                let parsed_id = id.as_str().parse::<i32>().unwrap();
                return Some(parsed_id);
            }
        }
    }
    return None;
}

async fn blog_exists(id: &i32) -> bool {
    let pool = get_db().await;
    match Blog::get_blog(&pool, *id).await {
        Ok(_) => return true,
        _ => return false,
    }
}

async fn metadata_to_blog(metadata: Metadata) -> Option<Blog> {
    // let settings = get_settings(None);

    if metadata.is_complete() {
        let id = metadata.id.unwrap().parse::<i32>().ok()?;
        let title = metadata.title.clone().unwrap();
        let body = metadata.body.clone().unwrap();
        let summary = String::from("TBD");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file(path: &str) -> String {
        let content =
            fs::read_to_string(path).expect(format!("Error reading from {:?}", path).as_str());
        content
    }

    #[tokio::test]
    async fn test_get_id() {
        let path = "articles/test.md";
        let content = read_file(path);
        let id = get_id(content.as_str())
            .await
            .expect("Couldn't find id in markdown file");
        assert_eq!(id, 420)
    }

    #[tokio::test]
    async fn test_if_nonexistent_blog_exists() {
        let id = 6969;
        let exists = blog_exists(&id).await;
        assert_eq!(exists, false)
    }
}
