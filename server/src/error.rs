use actix::MailboxError;
use actix_web::{
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Payload deserialization error")]
    CouldNotDeserializePayload,
    #[error("Could not read request body: {0}")]
    InvalidRequestBody(String),
    #[error("Invalid or missing github event header: {0}")]
    InvalidEventHeader(String),
    #[error("Could not deliver message because the inbox is full")]
    MailboxFull,
    #[error("Could not deliver message because the mailbox has closed")]
    MailboxClosed,
    #[error("Other mailbox error. {0}")]
    MailboxError(#[from] MailboxError),
    #[error("Rule configuration error: {0}")]
    RuleConfigurationError(String),
    #[error("An I/O error happened in the server. {0}")]
    IOError(#[from] std::io::Error),
    #[error("UnspecifiedError. {0}")]
    Unspecified(String),
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
            Self::InvalidRequestBody(_) => StatusCode::BAD_REQUEST,
            Self::InvalidEventHeader(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
