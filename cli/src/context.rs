use gh_pilot::data_provider::PullRequestProvider;
use gh_pilot::data_provider::UserStatsProvider;
use gh_pilot::mock_provider::{MockPRProvider, MockUserProvider};
use gh_pilot::GithubProvider;

pub struct Context {
    user_stats_provider: Box<dyn UserStatsProvider>,
    pr_provider: Box<dyn PullRequestProvider>,
    github_provider: Option<GithubProvider>,
}

impl Context {
    pub fn mock() -> Self {
        let user_stats_provider = Box::new(MockUserProvider::default());
        let pr_provider = Box::new(MockPRProvider {});
        Self {
            user_stats_provider,
            pr_provider,
            github_provider: None,
        }
    }

    pub fn custom_context() -> Self {
        let mut this = Self::mock();
        let github_provider = GithubProvider::default();
        this.github_provider = Some(github_provider);
        this
    }

    pub fn user_provider(&self) -> &dyn UserStatsProvider {
        self.user_stats_provider.as_ref()
    }

    pub fn pull_request_provider(&self) -> &dyn PullRequestProvider {
        if let Some(gh) = &self.github_provider {
            gh as &dyn PullRequestProvider
        } else {
            self.pr_provider.as_ref()
        }
    }
}
