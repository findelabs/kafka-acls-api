//use serde_json::error::Error as SerdeError;
use axum::{
    body::{self},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug)]
pub enum Error {
    Forbidden,
    Unauthorized,
    NotFound,
    Hyper(hyper::Error),
    Validation(ValidationError),
    Validations(ValidationErrors)
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Forbidden => f.write_str("{\"error\": \"Cannot get config: Forbidden\"}"),
            Error::Unauthorized => f.write_str("{\"error\": \"Cannot get config: Unauthorized\"}"),
            Error::NotFound => f.write_str("{\"error\": \"Cannot get config: Not found\"}"),
            Error::Validation(ref err) => write!(f, "{{\"error\": \"{}\"}}", err),
            Error::Hyper(ref err) => write!(f, "{{\"error\": \"{}\"}}", err),
            Error::Validations(ref err) => write!(f, "{{\"error\": \"{}\"}}", err)
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let payload = self.to_string();
        let body = body::boxed(body::Full::from(payload));

        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body)
            .unwrap()
    }
}

impl From<validator::ValidationError> for Error {
    fn from(err: validator::ValidationError) -> Error {
        Error::Validation(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Error {
        Error::Validations(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}
