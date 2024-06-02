use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::{instantiate_table_query, IURL},
        response::{
            connection_error, create_table_error, insert_conflict, insert_failure,
            invalid_form_data, response_success,
        },
        shortest::get_shortest_value,
    },
};
use chrono::{FixedOffset, Utc};
use regex::Regex;
use rocket::{form::Form, http::Status, serde::json::Json};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(FromForm)]
pub struct IBody {
    url: String,
}

#[post("/url", data = "<body>")]
pub async fn index(
    body: Option<Form<IBody>>,
) -> Result<(Status, Json<IBaseResponse<IURL>>), (Status, Json<IBaseResponse>)> {
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

    // Insert the url
    let rows = match sqlx::query_as!(
        IURL,
        r#"
        INSERT INTO url (short, long, date) 
        values ($1, $2, $3)
        RETURNING *
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
            data[0].clone()
        }
        Err(_err) => {
            return Err((Status::BadRequest, Json(insert_failure())));
        }
    };

    Ok((
        Status::Accepted,
        Json::<IBaseResponse<IURL>>(response_success(rows.clone())),
    ))
}
