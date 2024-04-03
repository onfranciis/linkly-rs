#[macro_use]
extern crate rocket;
mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Connected!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, controllers::get_all_url::index])
}
