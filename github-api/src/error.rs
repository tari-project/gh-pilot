use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubPilotError {
    #[error("Could not deserialize the event data. {0}")]
    EventDeserializationError(#[from] serde_json::Error),
    #[error("Unsupported or unknown event, {0}")]
    UnknownEvent(String),
    #[error("Could not convert Issue into desired type. {0}")]
    IssueConversionError(String),
}
