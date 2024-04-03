use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)] // Derive Serialize for IURL
pub struct IURL {
    id: String,
    url: String,
    short: String,
}

#[get("/url")]
pub fn index() -> Json<IURL> {
    let id: String = Uuid::new_v4().to_string();

    let formatted_data: IURL = IURL {
        id,
        url: String::from("www.onfranciis.dev"),
        short: String::from("odev"),
    };

    Json(formatted_data)
}
