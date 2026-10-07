use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::IURLWithoutID,
        response::{invalid_short, redis_500, serialisation_error},
    },
    RedisPool,
};

use rocket::{
    http::Status,
    response::Redirect,
    serde::json::{from_str, Json},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

#[get("/url/<short>")]
pub async fn index(
    mut redis: Connection<RedisPool>,
    short: &str,
) -> Result<Redirect, (Status, Json<IBaseResponse>)> {
    // Read value from redis
    match redis.get::<&str, Option<String>>(short).await {
        // Proceed if value exists
        Ok(Some(url)) => match from_str::<IURLWithoutID>(&url) {
            // Performs redirect if the value exists and is serialised properly
            Ok(value) => Ok(Redirect::permanent(value.long)),

            // Throw 400 if it fails to serialise it like IURLWithoutID
            Err(err) => {
                log::error!(
                    "Failed to deserialize URL for short '{}' from redis: {:?}",
                    short,
                    err
                );
                Err((Status::BadRequest, Json(serialisation_error())))
            }
        },

        // Throw 404 if it doesn't exist
        Ok(None) => Err((Status::NotFound, Json(invalid_short()))),

        // Throw 500 if there's a redis error
        Err(err) => {
            log::error!(
                "Redis connection error while fetching short '{}': {:?}",
                short,
                err
            );
            Err((Status::InternalServerError, Json(redis_500())))
        }
    }
}
