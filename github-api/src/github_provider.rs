use std::env;

use async_trait::async_trait;
use log::*;

use crate::{
    api::{AuthToken, ClientProxy, IssueRequest, PullRequestRequest, RepoRequest},
    error::GithubProviderError,
    models::{Contributor, Issue, Label, PullRequest, Repository, SimpleUser},
    provider_traits::{Contributors, IssueProvider, PullRequestProvider, RepoProvider, UserProvider},
    wrappers::{GithubHandle, IssueId, NewLabel},
};

pub const GITHUB_USER_ENVAR_NAME: &str = "GH_PILOT_USERNAME";
pub const GITHUB_AUTH_TOKEN_ENVAR_NAME: &str = "GH_PILOT_AUTH_TOKEN";

#[derive(Clone)]
pub struct GithubProvider {
    client: ClientProxy,
}

impl Default for GithubProvider {
    /// Create a default `GithubProvider` instance, which reads from envars if possible, or else creates a client
    /// with blank credentials
    fn default() -> Self {
        GithubProvider::from_environment().unwrap_or_else(|err| {
            warn!(
                "🐙 Could not create default Github Provider instance using environment variables. {}. Setting \
                 credentials to blank (and almost certainly incorrect) values.",
                err.to_string()
            );
            GithubProvider::new("user", "")
        })
    }
}

impl GithubProvider {
    pub fn new(username: &str, auth_token: &str) -> Self {
        let client = ClientProxy::new(username, auth_token.into());
        Self { client }
    }

    /// Create a GithubProvider instance by reading in variables from the environment. If the variables are missing,
    /// this method will return an error. The `Default` implementation uses this method, but will not fail,
    /// substituting blank values for username and auth token (which will typically fail).
    pub fn from_environment() -> Result<Self, GithubProviderError> {
        let username = env::var(GITHUB_USER_ENVAR_NAME).map_err(|_| {
            GithubProviderError::ConfigurationError(format!(
                "Cannot set github user name. Missing {} environment variable",
                GITHUB_USER_ENVAR_NAME
            ))
        })?;
        let token: AuthToken = env::var(GITHUB_AUTH_TOKEN_ENVAR_NAME)
            .map(AuthToken::from)
            .map_err(|_| {
                GithubProviderError::ConfigurationError(format!(
                    "Cannot set github auth token. Missing {} environment variable",
                    GITHUB_AUTH_TOKEN_ENVAR_NAME
                ))
            })?;
        let client = ClientProxy::new(username.as_str(), token);
        Ok(Self { client })
    }
}

#[async_trait]
impl PullRequestProvider for GithubProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubProviderError> {
        let pr = PullRequestRequest::new(owner, repo, number);
        let result = pr.fetch(&self.client).await?;
        Ok(result)
    }
}

#[async_trait]
impl IssueProvider for GithubProvider {
    async fn fetch_issue(&self, id: &IssueId) -> Result<Issue, GithubProviderError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let result = issue.fetch(&self.client).await?;
        Ok(result)
    }

    async fn add_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubProviderError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let labels = issue.add_labels(&[label], &self.client).await?;
        Ok(labels)
    }

    /// Remove a label from an issue.
    ///
    /// The boolean result indicates whether a call to the API to remove the label was actually made.
    /// e.g., if the label was not present on the issue, then no call was made, and the result is `false`.
    async fn remove_label(&self, id: &IssueId, label: &str, only_if_exists: bool) -> Result<bool, GithubProviderError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        if !only_if_exists || self.label_exists(label, id).await? {
            let _labels = issue.remove_label(label, &self.client).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    async fn label_exists(&self, label: &str, id: &IssueId) -> Result<bool, GithubProviderError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let labels = issue.fetch_labels(&self.client).await?;
        Ok(labels.iter().any(|l| l.name == label))
    }
}

#[async_trait]
impl UserProvider for GithubProvider {
    async fn fetch_details(&self, _handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubProviderError> {
        // TODO: implement
        Ok(None)
    }
}

#[async_trait]
impl RepoProvider for GithubProvider {
    async fn fetch_repository(&self, owner: &str, repo: &str) -> Result<Repository, GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let result = repo.fetch(&self.client).await?;
        Ok(result)
    }

    async fn fetch_labels(
        &self,
        owner: &str,
        repo: &str,
        page: Option<usize>,
        per_page: Option<usize>,
    ) -> Result<Vec<Label>, GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(100);
        let result = repo.fetch_labels(&self.client, page, per_page).await?;
        Ok(result)
    }

    async fn delete_label(&self, owner: &str, repo: &str, label: &str) -> Result<bool, GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let result = repo.delete_label(&self.client, label).await?;
        Ok(result)
    }

    async fn assign_labels(&self, owner: &str, repo: &str, labels: &[NewLabel]) -> Result<(), GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let result = repo.assign_labels(&self.client, labels).await?;
        Ok(result)
    }

    async fn edit_label(
        &self,
        owner: &str,
        repo: &str,
        label: &str,
        new: &NewLabel,
    ) -> Result<bool, GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let result = repo.edit_label(&self.client, label, new).await?;
        Ok(result)
    }
}

#[async_trait]
impl Contributors for GithubProvider {
    async fn get_contributors(&self, owner: &str, repo: &str) -> Result<Vec<Contributor>, GithubProviderError> {
        let repo = RepoRequest::new(owner, repo);
        let result = repo.fetch_contributors(&self.client).await?;
        Ok(result)
    }
}
