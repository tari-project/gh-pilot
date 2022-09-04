use std::fmt::{Display, Formatter};

use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ServerError {
    /// Invalid signature
    InvalidSignature,
    /// Payload deserialization error
    CouldNotDeserializePayload,
    /// Could not read request body: {0}
    InvalidRequestBody(String),
    /// Invalid or missing github event header: {0}
    InvalidEventHeader(String),
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InvalidSignature => StatusCode::UNAUTHORIZED,
            Self::CouldNotDeserializePayload => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidRequestBody(_) => StatusCode::BAD_REQUEST,
            Self::InvalidEventHeader(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}
