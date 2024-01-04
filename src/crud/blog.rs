use std::fs;

use sqlx::error::Error;
use sqlx::types::time::Date;
use sqlx::{Pool, Postgres};

#[derive(Debug, sqlx::FromRow)]
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
        let blogs: Vec<Blog> = sqlx::query_as::<_, Blog>("SELECT * FROM blog")
            .fetch_all(pool)
            .await?;

        Ok(blogs)
    }

    pub async fn get_blog(pool: &Pool<Postgres>, id: i32) -> Result<Blog, Error> {
        let mut blog: Blog = sqlx::query_as::<_, Blog>("SELECT * FROM blog WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        let file_path = format!("articles/blog{}.md", id);

        let content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                panic!("Failed to read file: {}", e);
            }
        };

        let html = markdown::to_html(&content);
        blog.body = html;

        Ok(blog)
    }

    pub async fn create_blog(pool: &Pool<Postgres>, blog: Blog) -> Result<Blog, Error> {
        // unimplemented!("Blog isn't creatable");
        let result = sqlx::query(
            "INSERT INTO blog (id, title, summary, body, date, tags)
            VALUES ($1, $2, $3, $4)",
        )
        .bind(blog.id)
        .bind(String::from("title").as_str())
        .bind(String::from("summary").as_str())
        .bind(String::from("body").as_str())
        .bind(blog.date)
        .bind(blog.tags.join(", "))
        .execute(pool)
        .await;

        println!("{:?}", result);

        Ok(blog)
    }
}
