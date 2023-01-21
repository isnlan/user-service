use axum::{http, Json};
use serde_json::json;

#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
    ServerError(String),
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status, msg) = (http::StatusCode::NOT_FOUND, "User not found");
        let body = Json(json!({
            "error": msg,
        }));

        (status, body).into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseQueryError(err) => {
                write!(f, "Database Query Error: {}", err)
            }
            Error::ServerError(err) => {
                write!(f, "Server Error: {}", err)
            }
        }
    }
}
