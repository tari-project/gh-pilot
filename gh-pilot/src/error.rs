use thiserror::Error;
use github::api::GithubApiError;

#[derive(Debug, Error)]
pub enum GithubPilotError {
    #[error("The Github API client encountered an error. {0}")]
    GithubError(#[from] GithubApiError),
    #[error("We encountered an unspecified problem in github pilot: {0}")]
    GeneralError(String),
}
