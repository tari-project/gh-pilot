use serde::{Deserialize, Serialize};

use crate::models::{CommonEventFields, Label, PullRequest, SimpleUser};

//------------------------------------    PullRequest Event     --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEvent {
    #[serde(flatten)]
    pub action: PullRequestAction,
    /// The pull request number.
    pub number: u64,
    pub pull_request: PullRequest,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "action")]
pub enum PullRequestAction {
    Assigned {
        assignee: SimpleUser,
    },
    AutoMergeDisabled,
    AutoMergeEnabled,
    /// If the action is closed and the merged key is false, the pull request was closed with unmerged commits. If
    /// the action is closed and the merged key is true, the pull request was merged.
    Closed,
    ConvertedToDraft,
    Edited {
        changes: PullRequestEditedChanges,
    },
    Labeled {
        label: Label,
    },
    Locked,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequestRemoved {
        requested_reviewer: SimpleUser,
    },
    ReviewRequested {
        requested_reviewer: SimpleUser,
    },
    /// Triggered when a pull request's head branch is updated. For example, when the head branch is updated from the
    /// base branch, when new commits are pushed to the head branch, or when the base branch is changed.
    Synchronize {
        after: String,
        before: String,
    },
    Unassigned {
        assignee: SimpleUser,
    },
    Unlabeled {
        label: Label,
    },
    Unlocked,
}

/// The changes to the comment if the action was `edited`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestEditedChangesBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<PullRequestEditedChangesTitle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChangesBody {
    /// The previous version of the body if the action was `edited`.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChangesTitle {
    /// The previous version of the title if the action was `edited`.
    pub from: String,
}
