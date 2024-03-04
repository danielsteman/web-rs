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
    pub async fn get_blogs(
        pool: &Pool<Postgres>,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Blog>, Error> {
        let query = format!("SELECT * FROM blog LIMIT {} OFFSET {}", limit, offset);
        let mut blogs: Vec<Blog> = sqlx::query_as::<_, Blog>(&query).fetch_all(pool).await?;

        blogs.sort_by(|a, b| b.id.cmp(&a.id));

        Ok(blogs)
    }

    pub async fn search_blogs(pool: &Pool<Postgres>, search: &str) -> Result<Vec<Blog>, Error> {
        let mut blogs: Vec<Blog> =
            sqlx::query_as::<_, Blog>("SELECT * FROM blog WHERE title ILIKE $1")
                .bind(format!("%{}%", search))
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

        let lines: Vec<&str> = blog.body.lines().collect();
        let content = lines.join("\n");

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

#[cfg(test)]
mod tests {
    use crate::utils::db::get_db;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_search() {
        let pool = get_db().await;
        let result = Blog::search_blogs(&pool, "hoi").await.unwrap();
        assert_eq!(result.len() > 0, true);
        assert_eq!(result[0].id, 420);
    }
}
