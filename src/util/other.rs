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

pub async fn setup() -> sqlx::Pool<Postgres> {
    dotenv().ok(); // Load .env file

    env::var("ROCKET_DATABASES")
        .expect("The 'ROCKET_DATABASES' variable was not found in this environment!");

    // Initialize PostgreSQL connection pool
    let database_url = env::var("DATABASE_URL")
        .expect("The 'DATABASE_URL' variable was not found in this environment!");

    let pg_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
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

    pg_pool
}
