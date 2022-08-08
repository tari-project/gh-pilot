use crate::error::GithubPilotError;
use async_trait::async_trait;
use github::api::{ClientProxy, IssueRequest, PullRequestRequest};
use github::models::{Issue, PullRequest};
use crate::data_provider::{IssueProvider, PullRequestProvider};

#[derive(Clone, Default)]
pub struct GithubProvider {
    client: ClientProxy,
}

impl GithubProvider {
    pub fn new(username: &str, auth_token: &str) -> Self {
        let client = ClientProxy::new(username, auth_token.into());
        Self { client }
    }
}

#[async_trait]
impl PullRequestProvider for GithubProvider {
    async fn fetch_pull_request(
        &self,
        owner: &str,
        repo: &str,
        number: u64,
    ) -> Result<PullRequest, GithubPilotError> {
        let pr = PullRequestRequest::new(owner, repo, number);
        let result = pr.fetch(&self.client).await?;
        Ok(result)
    }
}

#[async_trait]
impl IssueProvider for GithubProvider {
    async fn fetch_issue(&self, owner: &str, repo: &str, number: u64) -> Result<Issue, GithubPilotError> {
        let issue = IssueRequest::new(owner, repo, number);
        let result = issue.fetch(&self.client).await?;
        Ok(result)
    }
}
