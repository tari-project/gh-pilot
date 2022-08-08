use crate::error::GithubPilotError;
use async_trait::async_trait;
use github::models::{Issue, PullRequest};
use crate::models::{GithubHandle, UserDetails};

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
    ) -> Result<Option<UserDetails>, GithubPilotError>;
}

#[async_trait]
pub trait IssueProvider {
    async fn fetch_issue(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<Issue, GithubPilotError>;
}