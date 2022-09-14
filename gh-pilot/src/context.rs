use crate::data_provider::{IssueProvider, PullRequestProvider, UserStatsProvider};

#[derive(Default)]
pub struct Context<'app> {
    user_provider: Option<&'app dyn UserStatsProvider>,
    pr_provider: Option<&'app dyn PullRequestProvider>,
    issue_provider: Option<&'app dyn IssueProvider>,
}

impl<'app> Context<'app> {
    pub fn use_user_provider(&mut self, provider: &'app dyn UserStatsProvider) {
        self.user_provider = Some(provider);
    }

    pub fn user_provider(&self) -> Option<&dyn UserStatsProvider> {
        self.user_provider
    }

    pub fn use_pr_provider(&mut self, provider: &'app dyn PullRequestProvider) {
        self.pr_provider = Some(provider);
    }

    pub fn pull_request_provider(&self) -> Option<&dyn PullRequestProvider> {
        self.pr_provider
    }

    pub fn use_issue_provider(&mut self, provider: &'app dyn IssueProvider) {
        self.issue_provider = Some(provider);
    }

    pub fn issue_provider(&self) -> Option<&dyn IssueProvider> {
        self.issue_provider
    }
}
