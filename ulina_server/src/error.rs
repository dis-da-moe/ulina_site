#[derive(Debug)]
pub enum Error {
    NotFound,
    InternalError(String),
    InvalidPermissions(String),
}

impl From<rocket::Error> for Error {
    fn from(error: rocket::Error) -> Self {
        Error::InternalError(error.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Error::NotFound,
            _ => Error::InternalError(format!("{:?}", error)),
        }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;
        match self {
            NotFound => {
                format!("nation not found")
            }
            InternalError(error) => {
                format!("an internal error occurred: ```{}```", error)
            }
            InvalidPermissions(message) => message.clone(),
        }
    }
}
