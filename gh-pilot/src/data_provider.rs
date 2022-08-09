use crate::error::GithubPilotError;
use async_trait::async_trait;
use ghp_api::models::{Issue, Label, PullRequest, SimpleUser};
use crate::models::{GithubHandle, IssueId};

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubPilotError>;
}


#[async_trait]
pub trait UserStatsProvider {
    async fn fetch_details(
        &self,
        handle: &GithubHandle,
    ) -> Result<Option<SimpleUser>, GithubPilotError>;
}

#[async_trait]
pub trait IssueProvider {
    async fn fetch_issue(&self, id: &IssueId) -> Result<Issue, GithubPilotError>;

    async fn add_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubPilotError>;

    async fn remove_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubPilotError>;
}