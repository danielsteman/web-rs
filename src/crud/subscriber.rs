use sqlx::error::Error;
use sqlx::{Pool, Postgres};

#[derive(PartialEq, Debug, sqlx::FromRow)]
pub struct Subscriber {}

impl Subscriber {
    pub async fn create_subscriber(pool: &Pool<Postgres>, email: String) -> Result<(), Error> {
        let result = sqlx::query(
            "INSERT INTO subscriber (email)
			VALUES ($1)
			ON CONFLICT (email) DO NOTHING",
        )
        .bind(email)
        .execute(pool)
        .await?;

        println!("{:?}", result);

        Ok(())
    }

    pub async fn get_subscriber(pool: &Pool<Postgres>, email: String) -> Result<(), Error> {
        let result = sqlx::query("SELECT * FROM subscriber WHERE (email) = ($1)")
            .bind(email)
            .execute(pool)
            .await?;

        println!("{:?}", result);

        Ok(())
    }

    pub async fn delete_subscriber(pool: &Pool<Postgres>, email: String) -> Result<(), Error> {
        let result = sqlx::query(
            "DELETE FROM subscriber WHERE (email) = ($1)
			VALUES ($1)",
        )
        .bind(email)
        .execute(pool)
        .await?;

        println!("Deleted: {:?}", result);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::db::get_db;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_subscriber() {
        let pool = get_db().await;
        let email = "hoi@hoi.hoi";
        Subscriber::create_subscriber(&pool, String::from(email))
            .await
            .unwrap();
        // let res = subscriber.get_subscriber()
        // assert_eq!(result.len() > 0, true);
        // assert_eq!(result[0].id, 420);
    }
}
