use async_trait::async_trait;

use crate::{
    error::GithubProviderError,
    models::{Contributor, DateTime, Event, Label, Repository},
    wrappers::NewLabel,
};

#[async_trait]
pub trait RepoProvider {
    async fn fetch_repository(&self, owner: &str, repo: &str) -> Result<Repository, GithubProviderError>;
    // Label functionality
    async fn fetch_labels(
        &self,
        owner: &str,
        repo: &str,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Label>, GithubProviderError>;
    async fn delete_label(&self, owner: &str, repo: &str, label: &str) -> Result<bool, GithubProviderError>;
    async fn assign_labels(&self, owner: &str, repo: &str, labels: &[NewLabel]) -> Result<(), GithubProviderError>;
    async fn edit_label(
        &self,
        owner: &str,
        repo: &str,
        label: &str,
        new: &NewLabel,
    ) -> Result<bool, GithubProviderError>;
    async fn fetch_events(&self, owner: &str, repo: &str, since: DateTime) -> Result<Vec<Event>, GithubProviderError>;
}

#[async_trait]
pub trait Contributors {
    async fn fetch_contributors(&self, owner: &str, repo: &str) -> Result<Vec<Contributor>, GithubProviderError>;
}
