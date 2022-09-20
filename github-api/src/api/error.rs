use std::fmt::Debug;

use reqwest::StatusCode;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum GithubApiError {
    #[error("You need to provide an authorization token to access this endpoint. {0}")]
    Unauthorized(String),
    #[error(
        "The HTTP client returned an error. This is probably an issue with Github Pilot rather than the Github server \
         itself. {0}"
    )]
    HttpClientError(String),
    #[error("The request returned successfully, but deserialization from JSON failed. {0}")]
    DeserializationError(String),
    #[error("Could not serialize object to JSON. {0}")]
    SerializationError(String),
    #[error("The server said that this is not the URL you are looking for. {0}")]
    NotFound(String),
    #[error("HTTP Code response. {0}")]
    HttpResponse(StatusCode),
    #[error("Could not parse {0} as a valid timestamp")]
    InvalidTimestamp(String),
    #[error("Multiple errors occurred: {0:?}")]
    MultipleErrors(Vec<ErrorItem>),
    #[error("reqwest client error: {0}")]
    ReqwestError(String),
}

#[derive(Clone)]
pub struct ErrorItem {
    label: String,
    error: GithubApiError,
}

impl ErrorItem {
    pub fn new(label: &str, error: GithubApiError) -> Self {
        ErrorItem {
            label: label.into(),
            error,
        }
    }
}

impl Debug for ErrorItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.label, self.error)
    }
}
