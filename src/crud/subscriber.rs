use sqlx::error::Error;
use sqlx::{Pool, Postgres};

#[derive(PartialEq, Debug, sqlx::FromRow)]
pub struct Subscriber {
    pub email: String,
}

impl Subscriber {
    pub async fn create_subscriber(&self, pool: &Pool<Postgres>) -> Result<(), Error> {
        let result = sqlx::query(
            "INSERT INTO subscriber (email)
			VALUES ($1)
			ON CONFLICT (email) DO NOTHING",
        )
        .bind(&self.email)
        .execute(pool)
        .await?;

        println!("{:?}", result);

        Ok(())
    }
}
