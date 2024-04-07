use sqlx::error::Error;
use sqlx::{Pool, Postgres};

#[derive(PartialEq, Debug, sqlx::FromRow)]
pub struct Subscriber {
    email: String,
}

impl Subscriber {
    pub async fn create_subscriber(pool: &Pool<Postgres>, email: &str) -> Result<(), Error> {
        let existing_subscriber =
            sqlx::query!("SELECT email FROM subscriber WHERE email = $1", email)
                .fetch_optional(pool)
                .await?;

        if existing_subscriber.is_some() {
            println!("Subscriber with email {} already exists", email);
            return Ok(());
        }

        let result = sqlx::query(
            "INSERT INTO subscriber (email)
			VALUES ($1)",
        )
        .bind(email)
        .execute(pool)
        .await?;

        match result.rows_affected() {
            1 => {
                println!("Subscriber with email {} created", email);
                Ok(())
            }
            _ => Err(Error::RowNotFound),
        }
    }

    pub async fn get_subscriber(pool: &Pool<Postgres>, email: &str) -> Result<Subscriber, Error> {
        let subscriber: Subscriber =
            sqlx::query_as::<_, Subscriber>("SELECT * FROM subscriber WHERE email = $1")
                .bind(email)
                .fetch_one(pool)
                .await?;
        Ok(subscriber)
    }

    pub async fn get_subscribers(
        pool: &Pool<Postgres>,
        email: &str,
    ) -> Result<Vec<Subscriber>, Error> {
        let subscribers: Vec<Subscriber> =
            sqlx::query_as::<_, Subscriber>("SELECT * FROM subscriber WHERE email ILIKE $1")
                .bind(email)
                .fetch_all(pool)
                .await?;

        println!("{:?}", subscribers);

        Ok(subscribers)
    }

    pub async fn delete_subscriber(pool: &Pool<Postgres>, email: &str) -> Result<(), Error> {
        let result = sqlx::query("DELETE FROM subscriber WHERE email = $1")
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

    #[tokio::test]
    async fn test_subscriber() {
        let pool = get_db().await;
        let email = "hoi@hoi.hoi";
        Subscriber::create_subscriber(&pool, email).await.unwrap();
        let res = Subscriber::get_subscriber(&pool, email).await.unwrap();
        assert_eq!(res.email, email);
        Subscriber::delete_subscriber(&pool, email).await.unwrap();
    }
}
