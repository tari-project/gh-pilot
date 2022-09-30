use async_trait::async_trait;

use crate::{error::GithubProviderError, graphql::PullRequestComments, models::PullRequest, wrappers::IssueId};

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubProviderError>;
}

#[async_trait]
pub trait PullRequestCommentsProvider {
    async fn fetch_pull_request_comments(&self, pr_id: &IssueId) -> Result<PullRequestComments, GithubProviderError>;
}
