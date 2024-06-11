use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::{instantiate_table_query, OptionalIURL, IURL},
        response::{
            connection_error, create_table_error, insert_conflict, insert_failure,
            invalid_form_data, response_success, serialisation_error,
        },
        shortest::get_shortest_value,
    },
    RedisPool,
};
use chrono::{FixedOffset, Utc};
use regex::Regex;
use rocket::{
    form::Form,
    http::Status,
    serde::json::{to_string, Json},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(FromForm)]
pub struct IBody {
    url: String,
}

#[post("/url", data = "<body>")]
pub async fn index(
    mut redis: Connection<RedisPool>,
    body: Option<Form<IBody>>,
) -> Result<(Status, Json<IBaseResponse<OptionalIURL>>), (Status, Json<IBaseResponse>)> {
    let pg_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_err) => {
            return Err((Status::BadRequest, Json(connection_error())));
        }
    };

    let data = match body {
        None => return Err((Status::BadRequest, Json(invalid_form_data()))),
        Some(data) => data,
    };

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await
    {
        Ok(data) => data,
        Err(err) => {
            print!("{:?}", err);
            return Err((Status::BadRequest, Json(connection_error())));
        }
    };

    // Create a table
    match sqlx::query(&instantiate_table_query()).execute(&pool).await {
        Ok(_data) => (),
        Err(_err) => {
            return Err((Status::BadRequest, Json(create_table_error())));
        }
    };

    let regexp = Regex::new(r"^https?://").unwrap();
    let mut url = String::from(&data.url.clone());
    if !regexp.is_match(&url) {
        url = format!("http://{}", &url);
    }

    // Throw 400 if url attribute of body object does not exists
    if url.trim().is_empty() {
        return Err((Status::BadRequest, Json(invalid_form_data())));
    }

    // Check if it already exists
    match sqlx::query_as!(
        IURL,
        r#"
        SELECT * FROM url
        WHERE long = $1
        "#,
        url
    )
    .fetch_all(&pool)
    .await
    {
        Ok(data) => {
            print!("{:?}", data);

            // Throw 409 if it already exists
            if data.len() > 0 {
                return Err((Status::Conflict, Json(insert_conflict(data[0].clone()))));
            }
        }

        Err(_err) => (),
    };

    // Create uuid, split it and get the shortest word to use as the url short version
    let id: String = String::from(Uuid::new_v4());
    let uuid_array: Vec<&str> = id.split("-").collect::<Vec<&str>>();
    let short: String = get_shortest_value(uuid_array).to_string();

    // Add offset to make it GMT+1 i.e UTC+1
    let offset = FixedOffset::east_opt(1 * 60 * 60).unwrap();
    let curr_time = Utc::now()
        .with_timezone(&offset)
        .format("%d %b-%Y %H:%M:%S%P %z")
        .to_string();

    // Insert a new url and return all the urls
    let rows = match sqlx::query_as!(
        OptionalIURL,
        r#"
        WITH inserted_row AS (
        INSERT INTO url (short, long, date) 
        VALUES ($1, $2, $3)
        RETURNING  short, long, date
        )
        
        SELECT short, long, date FROM inserted_row
        UNION ALL
        SELECT  short,long, date FROM url;
        "#,
        short,
        url,
        curr_time
    )
    .fetch_all(&pool)
    .await
    {
        Ok(data) => {
            print!("{:?}", data);
            data.clone()
        }
        Err(_err) => {
            return Err((Status::BadRequest, Json(insert_failure())));
        }
    };

    fn convert_struct_to_string(
        rows: &OptionalIURL,
    ) -> Result<
        std::string::String,
        (
            rocket::http::Status,
            rocket::serde::json::Json<IBaseResponse>,
        ),
    > {
        match to_string(&rows) {
            Ok(data) => Ok(data),
            Err(_) => {
                println!(
                    "Something went wrong while trying to serialise {:?} to redis",
                    &rows
                );

                return Err((Status::BadRequest, Json(serialisation_error())));
            }
        }
    }

    fn convert_vector_to_string(
        rows: &Vec<OptionalIURL>,
    ) -> Result<
        std::string::String,
        (
            rocket::http::Status,
            rocket::serde::json::Json<IBaseResponse>,
        ),
    > {
        match to_string(&rows) {
            Ok(data) => Ok(data),
            Err(_) => {
                println!(
                    "Something went wrong while trying to serialise {:?} to redis",
                    &rows
                );

                return Err((Status::BadRequest, Json(serialisation_error())));
            }
        }
    }

    let individual_url_struct = OptionalIURL {
        short: Some(short.clone()),
        long: Some(url),
        date: Some(curr_time),
    };

    // Add individual url to redis
    match redis
        .set::<String, String, String>(
            short,
            convert_struct_to_string(&individual_url_struct).unwrap(),
        )
        .await
    {
        Ok(_) => {
            // Add all url to redis
            match redis
                .set::<&str, String, String>("urls", convert_vector_to_string(&rows).unwrap())
                .await
            {
                Ok(_) => Ok((
                    Status::Accepted,
                    Json::<IBaseResponse<OptionalIURL>>(response_success(individual_url_struct)),
                )),

                Err(_) => {
                    println!(
                        "Something went wrong while trying to add {:?} to redis",
                        rows
                    );

                    Err((Status::BadRequest, Json(insert_failure())))
                }
            }
        }

        Err(_) => {
            println!(
                "Something went wrong while trying to add {:?} to redis",
                rows
            );

            Err((Status::BadRequest, Json(insert_failure())))
        }
    }
}
