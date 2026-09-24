use crate::catchers::catchers::IBaseResponse;

use super::other::IURL;

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
        err: Some(format!(
            "Seems like this url has already been shortened! Is it '{}' ?",
            data.short
        )),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_success() {
        let resp = response_success("test-data");
        assert_eq!(resp.result, Some("test-data"));
        assert_eq!(resp.err, None);
    }

    #[test]
    fn test_insert_failure() {
        let resp = insert_failure();
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("Seems like there's a problem updating the DB!".to_string())
        );
    }

    #[test]
    fn test_get_query_failure() {
        let resp = get_query_failure();
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("Seems like there's a problem reading from the DB!".to_string())
        );
    }

    #[test]
    fn test_invalid_form_data() {
        let resp = invalid_form_data();
        assert_eq!(resp.result, None);
        assert_eq!(resp.err, Some("Invalid form format!".to_string()));
    }

    #[test]
    fn test_insert_conflict() {
        let mock_url = IURL {
            id: 1,
            long: "https://example.com".to_string(),
            short: "abc1".to_string(),
            date: "now".to_string(),
        };
        let resp = insert_conflict(mock_url);
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("Seems like this url has already been shortened! Is it 'abc1' ?".to_string())
        );
    }

    #[test]
    fn test_invalid_short() {
        let resp = invalid_short();
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("This url is invalid! Kindly confirm".to_string())
        );
    }

    #[test]
    fn test_redis_500() {
        let resp = redis_500();
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("Something went wrong with redis!".to_string())
        );
    }

    #[test]
    fn test_serialisation_error() {
        let resp = serialisation_error();
        assert_eq!(resp.result, None);
        assert_eq!(
            resp.err,
            Some("There was an error while serialising!".to_string())
        );
    }
}
