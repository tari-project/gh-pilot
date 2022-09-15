//! Additional implementations for PullRequestEvent are kept here to avoid messing with the code generation tools in
//! models.rs
use std::fmt::{Display, Formatter};

use crate::{
    api::PullRequestRequest,
    models::PullRequest,
    webhooks::{PullRequestAction, PullRequestEvent},
};

impl PullRequestEvent {
    pub fn owner(&self) -> &str {
        self.info.repository.owner.login.as_str()
    }

    pub fn number(&self) -> u64 {
        self.pull_request.number as u64
    }

    pub fn repo(&self) -> &str {
        self.info.repository.name.as_str()
    }

    pub fn pull_request(&self) -> &PullRequest {
        &self.pull_request
    }

    pub fn to_request(&self) -> PullRequestRequest {
        PullRequestRequest::new(self.owner(), self.repo(), self.number())
    }
}

impl Display for PullRequestAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PullRequestAction::Assigned { .. } => write!(f, "assigned"),
            PullRequestAction::Unassigned { .. } => write!(f, "unassigned"),
            PullRequestAction::Labeled { .. } => write!(f, "labeled"),
            PullRequestAction::Unlabeled { .. } => write!(f, "unlabeled"),
            PullRequestAction::Opened { .. } => write!(f, "opened"),
            PullRequestAction::Edited { .. } => write!(f, "edited"),
            PullRequestAction::Closed { .. } => write!(f, "closed"),
            PullRequestAction::Reopened { .. } => write!(f, "reopened"),
            PullRequestAction::Synchronize { .. } => write!(f, "synchronize"),
            PullRequestAction::ReadyForReview { .. } => write!(f, "ready_for_review"),
            PullRequestAction::Locked { .. } => write!(f, "locked"),
            PullRequestAction::Unlocked { .. } => write!(f, "unlocked"),
            PullRequestAction::ReviewRequested { .. } => write!(f, "review_requested"),
            PullRequestAction::ReviewRequestRemoved { .. } => write!(f, "review_request_removed"),
            PullRequestAction::ConvertedToDraft { .. } => write!(f, "converted_to_draft"),
            PullRequestAction::AutoMergeDisabled => write!(f, "auto_merge_disabled"),
            PullRequestAction::AutoMergeEnabled => write!(f, "auto_merge_enabled"),
        }
    }
}
