use crate::catchers::catchers::IBaseResponse;

use super::other::IURL;

pub fn connection_error() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from(
            "Invalid postgres connection string! Update your .env",
        )),
        result: None,
    }
}

pub fn create_table_error() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from("Could not create DB table!")),
        result: None,
    }
}

pub fn response_success<T>(result: T) -> IBaseResponse<T> {
    IBaseResponse {
        err: None,
        result: Some(result),
    }
}

pub fn insert_failure() -> IBaseResponse {
    IBaseResponse {
        result: None,
        err: Some(String::from(
            "Seems like there's a problem updating the DB!",
        )),
    }
}

pub fn get_query_failure() -> IBaseResponse {
    IBaseResponse {
        result: None,
        err: Some(String::from(
            "Seems like there's a problem reading from the DB!",
        )),
    }
}

pub fn invalid_form_data() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from("Invalid form format!")),
        result: None,
    }
}

pub fn insert_conflict(data: IURL) -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from(format!(
            "Seems like this url has already been shortened! Is it '{}' ?",
            data.short
        ))),
        result: None,
    }
}

pub fn invalid_short() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from("This url is invalid! Kindly confirm")),
        result: None,
    }
}

pub fn redis_500() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from("Something went wrong with redis!")),
        result: None,
    }
}

pub fn serialisation_error() -> IBaseResponse {
    IBaseResponse {
        err: Some(String::from("There was an error while serialising!")),
        result: None,
    }
}
