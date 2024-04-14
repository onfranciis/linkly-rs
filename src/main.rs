#[macro_use]
extern crate rocket;

mod catchers;
mod controllers;
mod util;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![catchers::catchers::index, controllers::get_all_url::index],
        )
        .register(
            "/",
            catchers![
                catchers::catchers::not_found,
                catchers::catchers::something_went_wrong
            ],
        )
}
