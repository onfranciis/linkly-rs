use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

use crate::catchers::catchers::IBaseResponse;

#[derive(Serialize)] // Derive Serialize for IURL
pub struct IURL {
    id: String,
    url: String,
    short: String,
}

#[get("/url")]
pub fn index() -> Json<IBaseResponse<self::IURL>> {
    let id: String = Uuid::new_v4().to_string();

    let formatted_data: IBaseResponse<self::IURL> = IBaseResponse::<self::IURL> {
        err: None,
        result: Some(self::IURL {
            id,
            url: String::from("www.onfranciis.dev"),
            short: String::from("odev"),
        }),
    };

    Json(formatted_data)
}
