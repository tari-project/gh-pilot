use serde::{Deserialize, Serialize};

use crate::models::{CommitComment, CommonEventFields, Issue, IssueComment, PullRequest, PullRequestReviewComment};

//----------------------------------      CommitComment Event     ------------------------------------------------------

/// A commit comment is created. The type of activity is specified in the `action` property.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitCommentEvent {
    /// The action performed. Can be `created`.
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommitCommentAction {
    #[serde(rename = "created")]
    Created,
}

//----------------------------------       IssueComment Event     ------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEvent {
    #[serde(flatten)]
    pub action: IssueCommentAction,
    pub comment: IssueComment,
    /// The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to.
    pub issue: Issue,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum IssueCommentAction {
    #[doc = "issue_comment created event"]
    #[serde(rename = "created")]
    Created,
    #[doc = "issue_comment deleted event"]
    #[serde(rename = "deleted")]
    Deleted,
    #[doc = "issue_comment edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        #[serde(skip_serializing_if = "Option::is_none")]
        comment: Option<IssueComment>,
    },
}

impl ToString for IssueCommentAction {
    fn to_string(&self) -> String {
        match *self {
            Self::Created => "created".to_string(),
            Self::Deleted => "deleted".to_string(),
            Self::Edited { .. } => "edited".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

/// The changes to the comment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssueCommentEditedChangesBody>,
}

//----------------------------   Pull-Request-Review-Comment Event    --------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEvent {
    #[serde(flatten)]
    pub action: PullRequestReviewCommentAction,
    pub pull_request: PullRequest,
    pub comment: PullRequestReviewComment,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", content = "changes")]
pub enum PullRequestReviewCommentAction {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "edited")]
    Edited(Box<PullRequestReviewCommentEditedChanges>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

/// The changes to the comment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestReviewCommentEditedChangesBody>,
}
