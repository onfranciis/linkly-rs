use chrono::{FixedOffset, Utc};
use mongodb::{Client, Collection};
use rocket::{form::Form, http::Status, serde::json::Json};
use serde::Serialize;
use uuid::Uuid;

use crate::{catchers::catchers::IBaseResponse, util::shortest::get_shortest_value};

#[derive(Serialize)]
pub struct IURL {
    id: String,
    url: String,
    short: String,
    date: String,
}

#[derive(FromForm)]
pub struct IBody {
    url: String,
}

#[post("/url", data = "<body>")]
pub async fn index(
    body: Option<Form<IBody>>,
) -> Result<(Status, Json<IBaseResponse<IURL>>), (Status, Json<IBaseResponse>)> {
    let invalid_form_data: IBaseResponse = IBaseResponse {
        err: Some(String::from("Invalid form format!")),
        result: None,
    };

    let connection_error: IBaseResponse = IBaseResponse {
        err: Some(String::from(
            "Invalid MongoDB connection string! Update your .env",
        )),
        result: None,
    };

    let mongodb_url = match std::env::var("MONGODB_URL") {
        Ok(url) => url,
        Err(_err) => {
            return Err((Status::BadRequest, Json(connection_error)));
        }
    };

    let client = match Client::with_uri_str(mongodb_url).await {
        Ok(client) => client,
        Err(err) => {
            println!("Error creating MongoDB client: {:?}", err);
            return Err((Status::BadRequest, Json(connection_error)));
        }
    };

    let url_collection: Collection<IURL> = client.database("linkly").collection("url");

    async fn proceed(
        data: Form<IBody>,
        invalid_form_data: IBaseResponse,
        url_collection: Collection<IURL>,
    ) -> Result<(Status, Json<IBaseResponse<IURL>>), (Status, Json<IBaseResponse>)> {
        let url: &String = &data.url;

        // Throw 400 if url attribute of body object does not exists
        if url.trim().is_empty() {
            return Err((Status::BadRequest, Json(invalid_form_data.clone())));
        }

        // Create uuid, split it get the shortest word to use as short version
        let id: String = String::from(Uuid::new_v4());
        let uuid_array: Vec<&str> = id.split("-").collect::<Vec<&str>>();
        let short: String = get_shortest_value(uuid_array).to_string();

        // Add offset to make it GMT+1 i.e UTC+1
        let offset = FixedOffset::east_opt(1 * 60 * 60).unwrap();
        let curr_time = Utc::now()
            .with_timezone(&offset)
            .format("%d %b-%Y %H:%M:%S %P %z");
        let doc = IURL {
            id: id.to_string(),
            url: String::from(url),
            short,
            date: curr_time.to_string(),
        };

        url_collection.insert_one(&doc, None).await.ok();

        let response = IBaseResponse::<IURL> {
            err: None,
            result: Some(doc),
        };

        return Ok((Status::Accepted, Json(response)));
    }

    match body {
        None => return Err((Status::BadRequest, Json(invalid_form_data.clone()))),
        Some(data) => {
            return proceed(data, invalid_form_data, url_collection).await;
        }
    }
}
