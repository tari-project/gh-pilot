use crate::error::GithubProviderError;
use crate::models::{Issue, Label};
use crate::wrappers::IssueId;
use async_trait::async_trait;

#[async_trait]
pub trait IssueProvider {
    async fn fetch_issue(&self, id: &IssueId) -> Result<Issue, GithubProviderError>;

    async fn add_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubProviderError>;

    async fn remove_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubProviderError>;
}