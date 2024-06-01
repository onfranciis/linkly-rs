use std::option::IntoIter;

use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::catchers::catchers::IBaseResponse;
use mongodb::{bson::doc, Client, Collection, Cursor};

#[derive(Serialize, Deserialize, Debug)]
pub struct IURL {
    id: String,
    url: String,
    short: String,
    date: Option<String>,
}
#[get("/url")]
pub async fn index() -> Result<(Status, Json<IBaseResponse>), (Status, Json<IBaseResponse>)> {
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
        url_collection: Collection<IURL>,
    ) -> Result<(Status, Json<IBaseResponse>), (Status, Json<IBaseResponse>)> {
        let cursor = match url_collection.find(doc! {"url":"google.com"}, None).await {
            Ok(result) => result,
            Err(err) => {
                print!("{:?}", (*err.kind));
                return Err((
                    Status::BadRequest,
                    Json(IBaseResponse {
                        err: Some(String::from("*err.kind")),
                        result: None,
                    }),
                ));
            }
        };

        let pre_response: IBaseResponse = IBaseResponse {
            err: None,
            result: Some(String::from("Hi")),
        };

        return Ok((Status::Accepted, Json(pre_response)));
    }

    proceed(url_collection).await
}
