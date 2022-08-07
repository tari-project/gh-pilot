use crate::data_provider::UserStatsProvider;
use crate::error::GithubPilotError;
use crate::models::{GithubHandle, UserDetails};
use async_trait::async_trait;
use github::models::static_data::users::*;

pub struct MockProvider {
    users: Vec<UserDetails>,
}

#[async_trait]
impl UserStatsProvider for MockProvider {
    async fn fetch_details(
        &self,
        handle: &GithubHandle,
    ) -> Result<Option<UserDetails>, GithubPilotError> {
        let user = self
            .users
            .iter()
            .find(|&u| u.login.as_str() == handle.as_ref())
            .cloned();
        Ok(user)
    }
}

impl Default for MockProvider {
    fn default() -> Self {
        let users: Vec<UserDetails> = vec![
            serde_json::from_str(CJS77).unwrap(),
            serde_json::from_str(STRINGHANDLER).unwrap()
        ];
        Self { users }
    }
}

#[cfg(test)]
mod test {
    use crate::data_provider::UserStatsProvider;
    use crate::mock_provider::MockProvider;

    #[tokio::test]
    async fn users_deserialize_correctly() {
        let provider = MockProvider::default();
        let user = provider
            .fetch_details(&"CjS77".into())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.name, "Cayle Sharrock");
        assert_eq!(user.followers, 65);
    }
}
