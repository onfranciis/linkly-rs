#[macro_use]
extern crate rocket;
use dotenv::dotenv;
use rocket_db_pools::{deadpool_redis, Database};

mod catchers;
mod controllers;
mod util;

#[derive(Database)]
#[database("redis_pool")]
pub struct RedisPool(deadpool_redis::Pool);

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Load .env file

    rocket::build()
        .attach(RedisPool::init())
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
                catchers::catchers::something_went_wrong
            ],
        )
}
