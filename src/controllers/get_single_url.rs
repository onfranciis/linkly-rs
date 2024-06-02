use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::{instantiate_table_query, IURL},
        response::{
            connection_error, create_table_error, get_query_failure, invalid_short,
            response_success,
        },
    },
};
use rocket::{http::Status, response::Redirect, serde::json::Json};
use sqlx::postgres::PgPoolOptions;

#[get("/url/<short>")]
pub async fn index(short: &str) -> Result<Redirect, (Status, Json<IBaseResponse>)> {
    let pg_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_err) => {
            return Err((Status::BadRequest, Json(connection_error())));
        }
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

    // Query the url
    match sqlx::query_as!(
        IURL,
        r#"
        SELECT * FROM url
        WHERE short = $1
        "#,
        short
    )
    .fetch_all(&pool)
    .await
    {
        Ok(rows) => {
            print!("{:?}", rows.len());

            if rows.len() > 0 {
                // use uri for internal redirect and
                // string for external redirect
                return Ok(Redirect::permanent(String::from("http://google.com")));

                // Ok((
                //     Status::Accepted,
                //     Json::<IBaseResponse<IURL>>(response_success(rows[0].clone())),
                // ))
            } else {
                return Err((Status::NotFound, Json(invalid_short())));
            }
        }
        Err(_err) => {
            return Err((Status::InternalServerError, Json(get_query_failure())));
        }
    }
}
