use crate::error::GithubPilotError;
use async_trait::async_trait;
use github::api::{ClientProxy, IssueRequest, PullRequestRequest};
use github::models::{Issue, Label, PullRequest};
use crate::data_provider::{IssueProvider, PullRequestProvider};
use crate::models::IssueId;

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
