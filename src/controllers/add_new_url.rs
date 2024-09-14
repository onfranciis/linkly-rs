use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::{current_time, OptionalIURL, IURL},
        response::{
            insert_conflict, insert_failure, invalid_form_data, response_success,
            serialisation_error,
        },
        shortest::get_shortest_value,
    },
    PgPool, RedisPool,
};
use regex::Regex;
use rocket::{
    form::Form,
    http::Status,
    serde::json::{to_string, Json},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use uuid::Uuid;

#[derive(FromForm)]
pub struct IBody {
    url: String,
}

#[post("/url", data = "<body>")]
pub async fn index(
    mut redis: Connection<RedisPool>,
    pg: &PgPool,
    body: Option<Form<IBody>>,
) -> Result<(Status, Json<IBaseResponse<OptionalIURL>>), (Status, Json<IBaseResponse>)> {
    let data = match body {
        None => return Err((Status::BadRequest, Json(invalid_form_data()))),
        Some(data) => data,
    };

    // Prepend the url with 'http' if it doesn't start with it
    let regexp = Regex::new(r"^https?://").unwrap();
    let mut url = String::from(&data.url.clone());

    // Throw 400 if url attribute of body object does not exists
    if url.trim().is_empty() {
        return Err((Status::BadRequest, Json(invalid_form_data())));
    }

    if !regexp.is_match(&url) {
        url = format!("http://{}", &url);
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
    .fetch_all(&**pg)
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
    let curr_time = current_time();

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
    .fetch_all(&**pg)
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
