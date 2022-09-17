use crate::error::GithubProviderError;
use crate::models::PullRequest;
use async_trait::async_trait;

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest, GithubProviderError>;
}