//! Additional implementations for PullRequestEvent are kept here to avoid messing with the code generation tools in
//! models.rs
use crate::{api::PullRequestRequest, models::PullRequest, webhooks::PullRequestEvent};

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
