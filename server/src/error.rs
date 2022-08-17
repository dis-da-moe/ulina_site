use common::TimeError;
use rocket::{serde::json, response::Responder};
use serenity::json::JsonError;
use sycamore::view;
use crate::site::rendering::Render;

#[derive(Debug, Clone)]
pub enum Error {
    NotFound,
    SocialNotFound(String),
    InternalError(String),
    InvalidPermissions(String),
    ExpectedImage(String),
    TooLarge(String),
    TimeError(TimeError),
    JsonError(String),
    NetworkError(String)
}

impl<'r> Responder<'r, 'static> for Error{
    fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let content = view!{(format!("error: {}", self.to_string()))}.render();

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

impl From<reqwest::Error> for Error{
    fn from(error: reqwest::Error) -> Self {
        Error::NetworkError(error.to_string())
    }
}

impl From<JsonError> for Error{
    fn from(e: JsonError) -> Self {
        Error::JsonError(e.to_string())
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
            TimeError(error) => {
                format!(
                    "an error occured while converting time: {}",
                    error.to_string()
                )
            }
            JsonError(e) => format!(
                "an error occured while deserialising JSON: {}",
                e
            ),
            NetworkError(e) => format!(
                "error occured while networking: {}",
                e
            )
        }
    }
}