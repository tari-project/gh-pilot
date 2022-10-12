use serde::{Deserialize, Serialize};

use crate::models::{Branch, Commit, CommonEventFields, Url};

//---------------------------------------    Status event   ------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// An array of branch objects containing the status' SHA. Each branch contains the given SHA, but the SHA
    /// may or may not be the head of the branch. The array includes a maximum of 10 branches.
    pub branches: Vec<Branch>,
    pub commit: Commit,
    pub context: String,
    pub created_at: String,
    /// The optional human-readable description added to the status.
    pub description: Option<String>,
    /// The unique identifier of the status.
    pub id: i64,
    pub name: String,
    /// The Commit SHA.
    pub sha: String,
    /// The new state. Can be `pending`, `success`, `failure`, or `error`.
    pub state: StatusEventState,
    /// The optional link added to the status.
    pub target_url: Option<Url>,
    pub updated_at: String,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

/// The new state. Can be `pending`, `success`, `failure`, or `error`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StatusEventState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "error")]
    Error,
}

impl ToString for StatusEventState {
    fn to_string(&self) -> String {
        match *self {
            StatusEventState::Pending => "pending".to_string(),
            StatusEventState::Success => "success".to_string(),
            StatusEventState::Failure => "failure".to_string(),
            StatusEventState::Error => "error".to_string(),
        }
    }
}
