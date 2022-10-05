use async_trait::async_trait;

use crate::{
    error::GithubProviderError,
    graphql::{review_counts::ReviewCounts, PullRequestComments},
    models::PullRequest,
    models_plus::{MergeParameters, MergeResult},
    wrappers::IssueId,
};

#[async_trait]
pub trait PullRequestProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubProviderError>;

    async fn merge_pull_request(
        &self,
        id: &IssueId,
        params: MergeParameters,
    ) -> Result<MergeResult, GithubProviderError>;
}

#[async_trait]
pub trait PullRequestCommentsProvider {
    async fn fetch_pull_request_comments(&self, pr_id: &IssueId) -> Result<PullRequestComments, GithubProviderError>;
}

#[async_trait]
pub trait PullRequestReviewSummary {
    async fn fetch_review_summary(&self, pr_id: &IssueId) -> Result<ReviewCounts, GithubProviderError>;
}
