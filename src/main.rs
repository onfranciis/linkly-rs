#[macro_use]
extern crate rocket;
use rocket_db_pools::{deadpool_redis, Database};
use util::other::setup;

mod catchers;
mod controllers;
mod util;

#[derive(Database)]
#[database("redis")]
pub struct RedisPool(deadpool_redis::Pool);

#[derive(Database)]
#[database("pg")]
pub struct PgPool(sqlx::PgPool);

#[launch]
async fn rocket() -> _ {
    let (redis_url, pg_url, pg_pool) = setup().await;

    let figment = rocket::Config::figment()
        .merge(("databases.redis.url", redis_url))
        .merge(("databases.pg.url", pg_url));

    // Use custom with figment instead of build() to bind pg and redis pool to env
    rocket::custom(figment)
        .attach(RedisPool::init())
        .attach(PgPool::init())
        .manage(pg_pool)
        .mount(
            "/",
            routes![
                catchers::catchers::index,
                controllers::get_all_url::index,
                controllers::add_new_url::index,
                controllers::get_single_url::index
            ],
        )
        .register(
            "/",
            catchers![
                catchers::catchers::not_found,
                catchers::catchers::something_went_wrong,
                catchers::catchers::service_unavailable
            ],
        )
}
