use rocket::{serde::json::Json, Request};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)] // Derive Serialize for IURL
pub struct IBaseResponse<T = String> {
    pub result: Option<T>,
    pub err: Option<String>,
}

#[derive(Serialize)] // Derive Serialize for IURL
pub struct IIndex {
    connected: bool,
}

#[get("/")]
pub fn index() -> Json<self::IIndex> {
    let formatted_data: IIndex = IIndex { connected: true };

    Json(formatted_data)
}

#[catch(404)]
pub fn not_found(_req: &Request) -> Json<IBaseResponse> {
    let response: IBaseResponse = IBaseResponse {
        err: Some(String::from("Invalid route!")),
        result: None,
    };

    Json(response)
}

#[catch(500)]
pub fn something_went_wrong(_req: &Request) -> Json<IBaseResponse> {
    let response: IBaseResponse = IBaseResponse {
        err: Some(String::from("Something went wrong!")),
        result: None,
    };

    Json(response)
}
