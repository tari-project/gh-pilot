use serde::{Deserialize, Serialize};

use crate::models::{AuthorAssociation, Links, Reactions, SimpleUser};

/// PullRequestReviewComment : Pull Request Review Comments are comments on a portion of the Pull Request's diff.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestReviewComment {
    /// URL for the pull request review comment
    pub url: String,
    /// The ID of the pull request review to which the comment belongs.
    pub pull_request_review_id: Option<i32>,
    /// The ID of the pull request review comment.
    pub id: i32,
    /// The node ID of the pull request review comment.
    pub node_id: String,
    /// The diff of the line that the comment refers to.
    pub diff_hunk: String,
    /// The relative path of the file to which the comment applies.
    pub path: String,
    /// The line index in the diff to which the comment applies. This field is deprecated; use `line` instead.
    pub position: Option<i32>,
    /// The index of the original line in the diff to which the comment applies. This field is deprecated; use
    /// `original_line` instead.
    pub original_position: Option<i32>,
    /// The SHA of the commit to which the comment applies.
    pub commit_id: String,
    /// The SHA of the original commit to which the comment applies.
    pub original_commit_id: String,
    /// The comment ID to reply to.
    pub in_reply_to_id: Option<i32>,
    #[serde(rename = "user")]
    pub user: SimpleUser,
    /// The text of the comment.
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
    /// HTML URL for the pull request review comment.
    pub html_url: String,
    /// URL for the pull request that the review comment belongs to.
    pub pull_request_url: String,
    pub author_association: AuthorAssociation,
    #[serde(rename = "_links")]
    pub links: Links,
    /// The first line of the range for a multi-line comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
    /// The first line of the range for a multi-line comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i32>,
    /// The side of the first line of the range for a multi-line comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_side: Option<Side>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    /// The line of the blob to which the comment applies. The last line of the range for a multi-line comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i32>,
    /// The side of the diff to which the comment applies. The side of the last line of the range for a multi-line
    /// comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Reactions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
}

/// The side of the first line of the range for a multi-line comment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}
