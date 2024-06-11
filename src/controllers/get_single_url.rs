use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::IURL,
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
    match redis.get::<&str, Option<String>>(short).await {
        Ok(Some(url)) => match from_str::<IURL>(&url) {
            Ok(value) => Ok(Redirect::permanent(value.long)),

            Err(err) => {
                print!("{:?}", err);
                return Err((Status::BadRequest, Json(serialisation_error())));
            }
        },

        Ok(None) => {
            return Err((Status::NotFound, Json(invalid_short())));
        }

        Err(err) => {
            print!("{:?}", err);
            return Err((Status::InternalServerError, Json(redis_500())));
        }
    }
}
