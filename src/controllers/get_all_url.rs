use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::{instantiate_table_query, IURL},
        response::{connection_error, create_table_error, get_query_failure, response_success},
    },
};
use rocket::{http::Status, serde::json::Json};
use sqlx::postgres::PgPoolOptions;

#[get("/url")]
pub async fn index(
) -> Result<(Status, Json<IBaseResponse<Vec<IURL>>>), (Status, Json<IBaseResponse>)> {
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
    let rows = match sqlx::query_as!(
        IURL,
        r#"
        SELECT * 
        FROM url
        "#
    )
    .fetch_all(&pool)
    .await
    {
        Ok(data) => data,
        Err(_err) => {
            return Err((Status::BadRequest, Json(get_query_failure())));
        }
    };

    Ok((
        Status::Accepted,
        Json::<IBaseResponse<Vec<IURL>>>(response_success(rows)),
    ))
}
