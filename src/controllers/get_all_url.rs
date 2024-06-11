use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::IURL,
        response::{get_query_failure, redis_500, response_success, serialisation_error},
    },
    RedisPool,
};
use rocket::{
    http::Status,
    serde::json::{from_str, Json},
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};

#[get("/url")]
pub async fn index(
    mut redis: Connection<RedisPool>,
) -> Result<(Status, Json<IBaseResponse<Vec<IURL>>>), (Status, Json<IBaseResponse>)> {
    match redis.get::<&str, Option<String>>("urls").await {
        Ok(Some(value)) => match from_str::<Vec<IURL>>(&value) {
            Ok(urls) => Ok((
                Status::Accepted,
                Json::<IBaseResponse<Vec<IURL>>>(response_success(urls)),
            )),

            Err(err) => {
                print!("{:?}", err);
                return Err((Status::BadRequest, Json(serialisation_error())));
            }
        },

        Ok(None) => {
            return Err((Status::BadRequest, Json(get_query_failure())));
        }

        Err(err) => {
            print!("{:?}", err);
            return Err((Status::InternalServerError, Json(redis_500())));
        }
    }
}
