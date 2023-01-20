#[derive(Debug)]
pub enum Error {
    DatabaseQueryError(sqlx::Error),
    ServerError(String),
}

impl actix_web::ResponseError for Error {}

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
