#[macro_use]
extern crate rocket;
extern crate mongodb;
use dotenv::dotenv;

mod catchers;
mod controllers;
mod util;

#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Load .env file

    rocket::build()
        .mount(
            "/",
            routes![
                catchers::catchers::index,
                controllers::get_all_url::index,
                controllers::add_new_url::index
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
