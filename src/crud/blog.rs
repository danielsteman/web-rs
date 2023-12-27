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
}
