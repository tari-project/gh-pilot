use crate::error::GithubPilotError;
use async_trait::async_trait;
use github::models::PullRequest;

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest, GithubPilotError>;
}