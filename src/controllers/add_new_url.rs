use rocket::{form::Form, http::Status, serde::json::Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{catchers::catchers::IBaseResponse, util::shortest::get_shortest_value};

#[derive(Serialize)]
pub struct IURL {
    id: String,
    url: String,
    short: String,
}

#[derive(FromForm)]
pub struct IBody {
    url: String,
}

#[post("/url", data = "<body>")]
pub fn index(
    body: Option<Form<IBody>>,
) -> Result<(Status, Json<IBaseResponse<self::IURL>>), (Status, Json<IBaseResponse>)> {
    let invalid_form_data: IBaseResponse = IBaseResponse {
        err: Some(String::from("Invalid form format!")),
        result: None,
    };

    let proceed = |data: Form<IBody>| {
        let url: &String = &data.url;

        // Throw 400 if url attribute of body object does not exists
        if url.trim().is_empty() {
            return Err((Status::BadRequest, Json(invalid_form_data.clone())));
        }

        // Create uuid, split it get the shortest word to use as short version
        let id: String = String::from(Uuid::new_v4());
        let uuid_array: Vec<&str> = id.split("-").collect::<Vec<&str>>();
        let short: String = get_shortest_value(uuid_array).to_string();

        let invalid_form_data: IBaseResponse<self::IURL> = IBaseResponse::<self::IURL> {
            err: None,
            result: Some(self::IURL {
                id: id.to_string(),
                url: String::from(url),
                short,
            }),
        };

        Ok((Status::Accepted, Json(invalid_form_data)))
    };

    match body {
        None => return Err((Status::BadRequest, Json(invalid_form_data.clone()))),
        Some(data) => {
            return proceed(data);
        }
    }
}
