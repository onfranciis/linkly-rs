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

/// Validates and normalizes the input URL.
/// Returns None if the URL is empty or consists solely of whitespace.
/// Prepends 'http://' if the URL does not start with 'http://' or 'https://'.
pub fn normalize_url(raw_url: &str) -> Option<String> {
    if raw_url.trim().is_empty() {
        return None;
    }

    let regexp = Regex::new(r"^https?://").unwrap();
    if !regexp.is_match(raw_url) {
        Some(format!("http://{}", raw_url))
    } else {
        Some(raw_url.to_string())
    }
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

    let url = match normalize_url(&data.url) {
        Some(u) => u,
        None => return Err((Status::BadRequest, Json(invalid_form_data()))),
    };

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
            log::debug!("Queried existing url records for '{}': {:?}", url, data);

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
            log::debug!("Inserted url row and fetched all urls: {:?}", data);
            data.clone()
        }
        Err(_err) => {
            log::error!("Database insert failure for url '{}': {:?}", url, _err);
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
            Err(err) => {
                log::error!(
                    "Something went wrong while trying to serialise {:?} to redis: {:?}",
                    &rows,
                    err
                );

                Err((Status::BadRequest, Json(serialisation_error())))
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
            Err(err) => {
                log::error!(
                    "Something went wrong while trying to serialise {:?} to redis: {:?}",
                    &rows,
                    err
                );

                Err((Status::BadRequest, Json(serialisation_error())))
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

                Err(err) => {
                    log::error!(
                        "Something went wrong while trying to add {:?} to redis: {:?}",
                        rows,
                        err
                    );

                    Err((Status::BadRequest, Json(insert_failure())))
                }
            }
        }

        Err(err) => {
            log::error!(
                "Something went wrong while trying to add {:?} to redis: {:?}",
                rows,
                err
            );

            Err((Status::BadRequest, Json(insert_failure())))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_url_empty_or_whitespace() {
        assert_eq!(normalize_url(""), None);
        assert_eq!(normalize_url("   "), None);
        assert_eq!(normalize_url("\t\n"), None);
    }

    #[test]
    fn test_normalize_url_without_scheme() {
        assert_eq!(
            normalize_url("example.com"),
            Some(String::from("http://example.com"))
        );
        assert_eq!(
            normalize_url("sub.domain.org/path?param=1"),
            Some(String::from("http://sub.domain.org/path?param=1"))
        );
    }

    #[test]
    fn test_normalize_url_with_http() {
        assert_eq!(
            normalize_url("http://example.com"),
            Some(String::from("http://example.com"))
        );
        assert_eq!(
            normalize_url("http://example.com/test"),
            Some(String::from("http://example.com/test"))
        );
    }

    #[test]
    fn test_normalize_url_with_https() {
        assert_eq!(
            normalize_url("https://example.com"),
            Some(String::from("https://example.com"))
        );
        assert_eq!(
            normalize_url("https://github.com/onfranciis/linkly-rs"),
            Some(String::from("https://github.com/onfranciis/linkly-rs"))
        );
    }
}
