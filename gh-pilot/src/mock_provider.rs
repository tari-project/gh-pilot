use async_trait::async_trait;
use ghp_api::models::{static_data::users::*, PullRequest, SimpleUser};

use crate::{
    data_provider::{PullRequestProvider, UserStatsProvider},
    error::GithubPilotError,
    models::GithubHandle,
};

pub struct MockUserProvider {
    users: Vec<SimpleUser>,
}

#[async_trait]
impl UserStatsProvider for MockUserProvider {
    async fn fetch_details(&self, handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubPilotError> {
        let user = self
            .users
            .iter()
            .find(|&u| u.login.as_str() == handle.as_ref())
            .cloned();
        Ok(user)
    }
}

impl Default for MockUserProvider {
    fn default() -> Self {
        let users: Vec<SimpleUser> = vec![
            serde_json::from_str(CJS77).unwrap(),
            serde_json::from_str(STRINGHANDLER).unwrap(),
        ];
        Self { users }
    }
}

pub struct MockPRProvider;

#[async_trait]
impl PullRequestProvider for MockPRProvider {
    async fn fetch_pull_request(
        &self,
        _owner: &str,
        _repo: &str,
        _number: u64,
    ) -> Result<PullRequest, GithubPilotError> {
        Err(GithubPilotError::GeneralError("Not implemented".to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::{data_provider::UserStatsProvider, mock_provider::MockUserProvider};

    #[tokio::test]
    async fn users_deserialize_correctly() {
        let provider = MockUserProvider::default();
        let user = provider.fetch_details(&"CjS77".into()).await.unwrap().unwrap();
        assert_eq!(user.name, "Cayle Sharrock");
        assert_eq!(user.followers, 65);
    }
}
