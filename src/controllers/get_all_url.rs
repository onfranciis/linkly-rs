use crate::{
    catchers::catchers::IBaseResponse,
    util::{
        other::OptionalIURL,
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
) -> Result<(Status, Json<IBaseResponse<Vec<OptionalIURL>>>), (Status, Json<IBaseResponse>)> {
    // Read "urls" from redis
    match redis.get::<&str, Option<String>>("urls").await {
        // If there is a value proceed
        Ok(Some(value)) => match from_str::<Vec<OptionalIURL>>(&value) {
            // If the value is a proper json of type OptionalIURL, return 200
            Ok(urls) => Ok((
                Status::Accepted,
                Json::<IBaseResponse<Vec<OptionalIURL>>>(response_success(urls)),
            )),

            // Throw 400 if it failed to be 'serialised'
            Err(err) => {
                print!("{:?}", err);
                return Err((Status::BadRequest, Json(serialisation_error())));
            }
        },

        // Throw 400 if it returned null
        Ok(None) => {
            return Err((Status::BadRequest, Json(get_query_failure())));
        }

        // Throw 500 if there's a redis connection problem
        Err(err) => {
            print!("{:?}", err);
            return Err((Status::InternalServerError, Json(redis_500())));
        }
    }
}
