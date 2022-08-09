use serde::{Deserialize, Serialize};
use crate::models::{AuthorAssociation, Links, SimpleUser};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestReview {
    /// Unique identifier of the review
    pub id: i32,
    pub node_id: String,
    pub user: Option<SimpleUser>,
    /// The text of the review.
    pub body: String,
    pub state: String,
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