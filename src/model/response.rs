use axum::{http, Json};
use serde::Serialize;
use serde_json::json;

const SUCCESS_CODE: i32 = 0;
const SUCCESS_MSG: &str = "ok";

#[derive(Serialize, Debug)]
pub struct Response<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: Option<T>) -> Self {
        Response {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_string(),
            data,
        }
    }
}

impl<T: Serialize> axum::response::IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!(self));

        (http::StatusCode::OK, body).into_response()
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseList<T>
where
    T: Serialize,
{
    pub total: i64,
    pub list: Vec<T>,
}
