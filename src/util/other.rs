use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IURL {
    pub id: i32,
    pub long: String,
    pub short: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OptionalIURL {
    pub long: Option<String>,
    pub short: Option<String>,
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IURLWithoutID {
    pub long: String,
    pub short: String,
    pub date: String,
}

pub async fn setup() -> (String, String, sqlx::Pool<Postgres>) {
    dotenv().ok(); // Load .env file

    let redis_url =
        env::var("REDIS_URL").expect("The 'REDIS_URL' variable was not found in this environment!");

    let pg_url = env::var("DATABASE_URL")
        .expect("The 'DATABASE_URL' variable was not found in this environment!");

    // Initialize PostgreSQL connection pool
    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await
        .expect("Failed to create pool.");

    // Create the table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS url (
            id SERIAL PRIMARY KEY,
            short VARCHAR(10) NOT NULL UNIQUE,
            long VARCHAR(255) NOT NULL UNIQUE,
            date VARCHAR(255) NOT NULL
        )
        "#,
    )
    .execute(&pg_pool)
    .await
    .expect("Something went wrong while instantiating the database!");

    (redis_url, pg_url, pg_pool)
}
