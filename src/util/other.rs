use chrono::{FixedOffset, Utc};
use dotenv::dotenv;
use redis::Commands;
use rocket::serde::json::to_string;
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

pub fn current_time() -> String {
    // Add offset to make it GMT+1 i.e UTC+1
    let offset = FixedOffset::east_opt(1 * 60 * 60).unwrap();
    let curr_time = Utc::now()
        .with_timezone(&offset)
        .format("%d %b-%Y %H:%M:%S%P %z")
        .to_string();

    curr_time
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
    setup_redis(&redis_url).await;

    (redis_url, pg_url, pg_pool)
}

pub async fn setup_redis(redis_url: &String) -> () {
    let client = redis::Client::open(redis_url.as_str()).expect("Could not open redis connection!");
    let mut conn = client
        .get_connection()
        .expect("Could not connect to Redis!");
    let demo_key = "demo";
    let demo_data = vec![IURLWithoutID {
        date: current_time(),
        short: String::from(demo_key),
        long: String::from("https://github.com/onfranciis/linkly-rs"),
    }];

    // Check if urls key exist in redis
    match conn.get::<&str, Option<Vec<String>>>("urls") {
        Ok(None) => {
            // If it doesn't exist? Create urls key and attatch demo data to it
            match conn.set::<&str, String, String>("urls", to_string(&demo_data).unwrap()) {
                Ok(_) => println!("Instantiated redis with {:?}", &demo_data),

                Err(_) => {
                    panic!(
                        "Something went wrong while trying to add {:?} to redis",
                        &demo_data
                    );
                }
            }
        }

        Ok(Some(_)) => (),

        Err(_) => panic!("Something went wrong while instantiating redis!"),
    }

    // Set demo key to match demo data
    match conn.set::<&str, String, String>("demo", to_string(&demo_data[0]).unwrap()) {
        Ok(_) => println!(
            "Added {:?} to redis with key {:?}",
            &demo_data[0], &demo_key
        ),

        Err(_) => {
            panic!(
                "Something went wrong while trying to add {:?} to key '{:?}' in redis!",
                &demo_data, &demo_key
            );
        }
    }
}
