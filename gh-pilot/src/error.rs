use ghp_api::api::GithubApiError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GithubPilotError {
    #[error("The Github API client encountered an error. {0}")]
    GithubError(#[from] GithubApiError),
    #[error("We encountered an unspecified problem in github pilot: {0}")]
    GeneralError(String),
    #[error("Configuration error. {0}")]
    ConfigurationError(String),
}
