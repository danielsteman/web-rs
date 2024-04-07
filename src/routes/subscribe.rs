use axum::extract::Form;
use axum::extract::State;
use serde::Deserialize;
use sqlx::PgPool;

use crate::crud::subscriber::Subscriber;

#[derive(Deserialize, Debug)]
pub struct Subscribe {
    email: String,
}

pub async fn subscribe(State(pool): State<PgPool>, Form(body): Form<Subscribe>) {
    match Subscriber::create_subscriber(&pool, body.email.as_str()).await {
        Ok(subscriber) => {
            println!("New subscriber: {}", body.email)
        }
        Err(error) => {
            println!("Error fetching blogs: {}", error);
        }
    }
}
