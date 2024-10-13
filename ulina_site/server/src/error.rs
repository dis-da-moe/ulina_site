use std::fmt::Debug;

use crate::site::rendering::Render;
use chrono::Utc;
use common::{ChangeTypeParseError, TimeError, DATE_TIME_FORMAT};
use rocket::response::Responder;
use serenity::json::JsonError;
use sycamore::view;

//TODO: rework this
#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    SocialNotFound(String),
    InternalError(String),
    InvalidPermissions(String),
    NotContinent(String),
    ExpectedImage(String),
    TooLarge(String),
    TimeError(TimeError),
    JsonError(String),
    NetworkError(String),
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let message = match self {
            Error::InternalError(e) => {
                println!(
                    "internal error while responding to request: {}",
                    e.to_string()
                );
                "Internal error occurred".to_string()
            }
            _ => self.to_string(),
        };

        let content = view! {(format!("error: {}", message))}.render();

        content.respond_to(request)
    }
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

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::NetworkError(error.to_string())
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Error::JsonError(e.to_string())
    }
}

impl From<ChangeTypeParseError> for Error {
    fn from(e: ChangeTypeParseError) -> Self {
        Error::InternalError(format!("invalid database entry: {}", e.0))
    }
}

#[macro_export]
macro_rules! internal {
    () => {
        |error| crate::error::Error::InternalError(format!("{:?}", error))
    };
}

impl ToString for Error {
    fn to_string(&self) -> String {
        use Error::*;
        match self {
            NotFound => {
                format!("nation not found")
            }
            InternalError(error) => {
                println!("{}: {}", Utc::now().format(DATE_TIME_FORMAT), error);
                "An internal error occurred".to_string()
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
            TimeError(error) => {
                format!(
                    "an error occured while converting time: {}",
                    error.to_string()
                )
            },
            NotContinent(continent) => {
                format!("\"{}\" is not a continent", continent)
            }
            JsonError(e) => format!("an error occured while deserialising JSON: {}", e),
            NetworkError(e) => format!("error occured while networking: {}", e),
        }
    }
}
