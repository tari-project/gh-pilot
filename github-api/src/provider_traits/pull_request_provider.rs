use async_trait::async_trait;

use crate::{error::GithubProviderError, models::PullRequest};

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubProviderError>;
}
