use gh_pilot::data_provider::UserStatsProvider;
use gh_pilot::mock_provider::MockProvider;

pub struct Context {
    user_stats_provider: Box<dyn UserStatsProvider>,
}

impl Context {
    pub fn mock() -> Self {
        let user_stats_provider = Box::new(MockProvider::default());
        Self {
            user_stats_provider,
        }
    }

    pub fn user_provider(&self) -> &dyn UserStatsProvider {
        self.user_stats_provider.as_ref()
    }
}
