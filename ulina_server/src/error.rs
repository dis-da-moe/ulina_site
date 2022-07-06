#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    SocialNotFound(String),
    InternalError(String),
    InvalidPermissions(String),
    ExpectedImage(String),
    TooLarge(String),
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
            ExpectedImage(received) => {
                format!("expected an image but got {}", received)
            }
            TooLarge(max) => {
                format!("the image is too large, the maximum size is {}", max)
            }
            SocialNotFound(platform) => {
                format!("could not find social on platform \"{}\"", platform)
            }
        }
    }
}
