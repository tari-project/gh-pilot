use serde::{Deserialize, Serialize};

use crate::models::{AuthorAssociation, Links, SimpleUser};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestReview {
    /// Unique identifier of the review
    pub id: i32,
    pub node_id: String,
    pub user: Option<SimpleUser>,
    /// The text of the review.
    #[serde(
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::models_plus::default_if_null"
    )]
    pub body: String,
    pub state: ReviewState,
    pub html_url: String,
    pub pull_request_url: String,
    #[serde(rename = "_links")]
    pub links: Links,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// A commit SHA for the review.
    pub commit_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    pub author_association: AuthorAssociation,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum ReviewState {
    /// A review allowing the pull request to merge.
    #[serde(rename = "approved")]
    Approved,
    /// A review blocking the pull request from merging.
    #[serde(rename = "changes_requested")]
    ChangesRequested,
    /// An informational review.
    #[serde(rename = "commented")]
    Commented,
    /// A review that has been dismissed.
    #[serde(rename = "dismissed")]
    Dismissed,
    /// A review that has not yet been submitted.
    #[serde(rename = "pending")]
    Pending,
}

impl std::fmt::Display for ReviewState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewState::Approved => write!(f, "Approved"),
            ReviewState::ChangesRequested => write!(f, "Changes requested"),
            ReviewState::Commented => write!(f, "Commented"),
            ReviewState::Dismissed => write!(f, "Dismissed"),
            ReviewState::Pending => write!(f, "Pending"),
        }
    }
}
