use std::fs;

use sqlx::error::Error;
use sqlx::types::time::Date;
use sqlx::{Pool, Postgres};

#[derive(PartialEq, Debug, sqlx::FromRow)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub date: Date,
    pub tags: Vec<String>,
}

impl Blog {
    pub async fn get_blogs(pool: &Pool<Postgres>) -> Result<Vec<Blog>, Error> {
        let mut blogs: Vec<Blog> = sqlx::query_as::<_, Blog>("SELECT * FROM blog")
            .fetch_all(pool)
            .await?;

        blogs.sort_by(|a, b| b.id.cmp(&a.id));

        Ok(blogs)
    }

    pub async fn search_blogs(pool: &Pool<Postgres>, search: &str) -> Result<Vec<Blog>, Error> {
        let mut blogs: Vec<Blog> = sqlx::query_as::<_, Blog>(
            format!("SELECT * FROM blog WHERE title LIKE %{}%", search).as_str(),
        )
        .fetch_all(pool)
        .await?;

        blogs.sort_by(|a, b| b.id.cmp(&a.id));

        Ok(blogs)
    }

    pub async fn get_blog(pool: &Pool<Postgres>, id: i32) -> Result<Blog, Error> {
        let mut blog: Blog = sqlx::query_as::<_, Blog>("SELECT * FROM blog WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let file_path = format!("articles/blog{}.md", id);

        let content = match fs::read_to_string(&file_path) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                let content_without_metadata = lines[4..].to_vec();
                let remaining_content = content_without_metadata.join("\n");
                remaining_content
            }
            Err(e) => {
                panic!("Failed to read file: {}", e);
            }
        };

        let html = markdown::to_html(&content);
        blog.body = html;

        Ok(blog)
    }

    pub async fn create_blog(&self, pool: &Pool<Postgres>) -> Result<(), Error> {
        let tags_array: Vec<&str> = self.tags.iter().map(|s| s.as_str()).collect();

        let result = sqlx::query(
            "INSERT INTO blog (id, title, summary, body, date, tags)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO NOTHING",
        )
        .bind(&self.id)
        .bind(&self.title)
        .bind(&self.summary)
        .bind(&self.body)
        .bind(&self.date)
        .bind(&tags_array)
        .execute(pool)
        .await?;

        println!("{:?}", result);

        Ok(())
    }
}
