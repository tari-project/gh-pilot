use async_trait::async_trait;

use crate::{
    error::GithubProviderError,
    models::{Issue, Label},
    wrappers::IssueId,
};

#[async_trait]
pub trait IssueProvider {
    async fn fetch_issue(&self, id: &IssueId) -> Result<Issue, GithubProviderError>;

    async fn add_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubProviderError>;

    async fn remove_label(&self, id: &IssueId, label: &str, only_if_exists: bool) -> Result<bool, GithubProviderError>;

    async fn label_exists(&self, label: &str, id: &IssueId) -> Result<bool, GithubProviderError>;

    async fn fetch_issue_labels(&self, id: &IssueId) -> Result<Vec<Label>, GithubProviderError>;
}
