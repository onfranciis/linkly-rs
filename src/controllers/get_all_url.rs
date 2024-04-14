use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::{catchers::catchers::IBaseResponse, util::shortest::get_shortest_value};

#[derive(Serialize)] // Derive Serialize for IURL
pub struct IURL {
    id: String,
    url: String,
    short: String,
}

#[get("/url")]
pub fn index() -> Json<IBaseResponse<self::IURL>> {
    let id: String = String::from(Uuid::new_v4());
    let uuid_array: Vec<&str> = id.split("-").collect::<Vec<&str>>();
    let short: String = get_shortest_value(uuid_array).to_string();

    let formatted_data: IBaseResponse<self::IURL> = IBaseResponse::<self::IURL> {
        err: None,
        result: Some(self::IURL {
            id: id.to_string(),
            url: String::from("www.onfranciis.dev"),
            short,
        }),
    };

    Json(formatted_data)
}
