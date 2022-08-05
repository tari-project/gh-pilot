use crate::data_provider::UserStatsProvider;
use crate::error::GithubPilotError;
use crate::models::{GithubHandle, UserDetails};
use async_trait::async_trait;

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
            serde_json::from_str(r#"
            {
                "login": "CjS77",
                "id": 7573551,
                "node_id": "MDQ6VXNlcjc1NzM1NTE=",
                "avatar_url": "https://avatars.githubusercontent.com/u/7573551?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/CjS77",
                "html_url": "https://github.com/CjS77",
                "followers_url": "https://api.github.com/users/CjS77/followers",
                "following_url": "https://api.github.com/users/CjS77/following{/other_user}",
                "gists_url": "https://api.github.com/users/CjS77/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/CjS77/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/CjS77/subscriptions",
                "organizations_url": "https://api.github.com/users/CjS77/orgs",
                "repos_url": "https://api.github.com/users/CjS77/repos",
                "events_url": "https://api.github.com/users/CjS77/events{/privacy}",
                "received_events_url": "https://api.github.com/users/CjS77/received_events",
                "type": "User",
                "site_admin": false,
                "name": "Cayle Sharrock",
                "company": null,
                "blog": "",
                "location": "Portugal",
                "email": null,
                "hireable": null,
                "bio": null,
                "twitter_username": null,
                "public_repos": 56,
                "public_gists": 10,
                "followers": 65,
                "following": 0,
                "created_at": "2014-05-13T20:06:19Z",
                "updated_at": "2022-08-02T10:53:39Z"
            }"#).unwrap(),
            serde_json::from_str(r#"
            {
                "login": "stringhandler",
                "id": 4200336,
                "node_id": "MDQ6VXNlcjQyMDAzMzY=",
                "avatar_url": "https://avatars.githubusercontent.com/u/4200336?v=4",
                "gravatar_id": "",
                "url": "https://api.github.com/users/stringhandler",
                "html_url": "https://github.com/stringhandler",
                "followers_url": "https://api.github.com/users/stringhandler/followers",
                "following_url": "https://api.github.com/users/stringhandler/following{/other_user}",
                "gists_url": "https://api.github.com/users/stringhandler/gists{/gist_id}",
                "starred_url": "https://api.github.com/users/stringhandler/starred{/owner}{/repo}",
                "subscriptions_url": "https://api.github.com/users/stringhandler/subscriptions",
                "organizations_url": "https://api.github.com/users/stringhandler/orgs",
                "repos_url": "https://api.github.com/users/stringhandler/repos",
                "events_url": "https://api.github.com/users/stringhandler/events{/privacy}",
                "received_events_url": "https://api.github.com/users/stringhandler/received_events",
                "type": "User",
                "site_admin": false,
                "name": "stringhandler",
                "blog": "",
                "location": null,
                "email": null,
                "hireable": null,
                "bio": null,
                "twitter_username": null,
                "public_repos": 39,
                "public_gists": 2,
                "followers": 14,
                "following": 6,
                "created_at": "2013-04-19T10:15:35Z",
                "updated_at": "2022-05-28T16:52:48Z"
            }
            "#).unwrap()
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
