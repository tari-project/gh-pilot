use github::api::{ClientProxy, PullRequestRequest};
use github::models::PullRequest;
use crate::data_provider::PullRequestProvider;
use crate::error::GithubPilotError;
use async_trait::async_trait;

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
    async fn fetch_pull_request(&self, owner: &str, repo: &str, number: u64) -> Result<PullRequest, GithubPilotError> {
        let pr = PullRequestRequest::new(owner, repo, number);
        let result = pr.fetch(&self.client).await?;
        Ok(result)
    }
}