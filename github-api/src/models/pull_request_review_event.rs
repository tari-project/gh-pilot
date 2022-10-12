use serde::{Deserialize, Serialize};

use crate::models::{CommonEventFields, PullRequest, PullRequestReview};

//----------------------------------  Pull-Request-Review Event   ------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEvent {
    #[serde(flatten)]
    pub action: PullRequestReviewAction,
    pub review: PullRequestReview,
    pub pull_request: PullRequest,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum PullRequestReviewAction {
    /// pull_request_review dismissed event
    #[serde(rename = "dismissed")]
    Dismissed,
    /// pull_request_review edited event
    #[serde(rename = "edited")]
    Edited { changes: PullRequestReviewEditedChanges },
    /// pull_request_review submitted event
    #[serde(rename = "submitted")]
    Submitted,
}

impl ToString for PullRequestReviewAction {
    fn to_string(&self) -> String {
        match self {
            Self::Dismissed => "dismissed".into(),
            Self::Edited { .. } => "edited".into(),
            Self::Submitted => "submitted".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEditedChanges {
    pub body: Option<PullRequestReviewEditedChangesBody>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEditedChangesBody {
    /// The previous version of the body if the action was `edited`.
    pub from: String,
}
