use thiserror::Error;
use crate::api::GithubApiError;

#[derive(Error, Debug)]
pub enum GithubProviderError {
    #[error("Could not deserialize the event data. {0}")]
    EventDeserializationError(#[from] serde_json::Error),
    #[error("Unsupported or unknown event, {0}")]
    UnknownEvent(String),
    #[error("Could not convert Issue into desired type. {0}")]
    IssueConversionError(String),
    #[error("The Github API client encountered an error. {0}")]
    GithubApiError(#[from] GithubApiError),
    #[error("We encountered an unspecified problem in github pilot: {0}")]
    GeneralError(String),
    #[error("Configuration error. {0}")]
    ConfigurationError(String),
}
