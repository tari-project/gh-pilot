use thiserror::Error;

#[derive(Debug, Error)]
pub enum GithubPilotError {
    #[error("We encountered an unspecified problem in github pilot: {0}")]
    GeneralError(String),
}
