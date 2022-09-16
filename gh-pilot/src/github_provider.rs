use std::env;

use async_trait::async_trait;
use ghp_api::{
    api::{AuthToken, ClientProxy, IssueRequest, PullRequestRequest},
    models::{Issue, Label, PullRequest, SimpleUser},
};
use log::*;

use crate::{
    data_provider::{IssueProvider, PullRequestProvider, UserStatsProvider},
    error::GithubPilotError,
    models::{GithubHandle, IssueId},
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
                "Could not create default Github Provider instance using environment variables. {}. Setting \
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
    pub fn from_environment() -> Result<Self, GithubPilotError> {
        let username = env::var(GITHUB_USER_ENVAR_NAME).map_err(|_| {
            GithubPilotError::ConfigurationError(format!(
                "Cannot set github user name. Missing {} environment variable",
                GITHUB_USER_ENVAR_NAME
            ))
        })?;
        let token: AuthToken = env::var(GITHUB_AUTH_TOKEN_ENVAR_NAME)
            .map(AuthToken::from)
            .map_err(|_| {
                GithubPilotError::ConfigurationError(format!(
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
    async fn fetch_pull_request(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest, GithubPilotError> {
        let pr = PullRequestRequest::new(owner, repo, number);
        let result = pr.fetch(&self.client).await?;
        Ok(result)
    }
}

#[async_trait]
impl IssueProvider for GithubProvider {
    async fn fetch_issue(&self, id: &IssueId) -> Result<Issue, GithubPilotError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let result = issue.fetch(&self.client).await?;
        Ok(result)
    }

    async fn add_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubPilotError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let labels = issue.add_labels(&[label], &self.client).await?;
        Ok(labels)
    }

    async fn remove_label(&self, id: &IssueId, label: &str) -> Result<Vec<Label>, GithubPilotError> {
        let issue = IssueRequest::new(&id.owner, &id.repo, id.number);
        let labels = issue.remove_label(label, &self.client).await?;
        Ok(labels)
    }
}

#[async_trait]
impl UserStatsProvider for GithubProvider {
    async fn fetch_details(&self, _handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubPilotError> {
        // TODO: implement
        Ok(None)
    }
}
