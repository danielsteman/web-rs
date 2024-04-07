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

    pub async fn get_subscribers(
        pool: &Pool<Postgres>,
        email: String,
    ) -> Result<Vec<Subscriber>, Error> {
        let subscribers: Vec<Subscriber> =
            sqlx::query_as::<_, Subscriber>("SELECT * FROM subscriber WHERE email ILIKE $1")
                .bind(email)
                .fetch_all(pool)
                .await?;

        println!("{:?}", subscribers);

        Ok(subscribers)
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
        let res = Subscriber::get_subscribers(&pool, String::from(email))
            .await
            .unwrap();
        assert_eq!(res.len() > 0, true);
    }
}
