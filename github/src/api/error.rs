use reqwest::StatusCode;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum GithubApiError {
    #[error("You need to provide an authorization token to access this endpoint. {0}")]
    Unauthorized(String),
    #[error("The HTTP client returned an error. This is probably an issue with Github Pilot rather than the Github \
    server itself. {0}")]
    HttpClientError(String),
    #[error("The request returned successfully, but serialization to JSON failed. {0}")]
    DeserializationError(String),
    #[error("The server said that this is not the URL you are looking for. {0}")]
    NotFound(String),
    #[error("HTTP Code response. {0}")]
    HttpResponse(StatusCode)
}