use serde::Deserialize;

/// Information about a user.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UserDetails {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub name: String,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<String>,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub public_repos: i64,
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Information about the current authenticated user.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthenticatedUser {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub site_admin: bool,

    // Extended over `User`:
    pub name: Option<String>,
    pub company: Option<String>,
    pub blog: String,
    pub location: Option<String>,
    pub email: Option<String>,
    pub hireable: Option<bool>,
    pub bio: Option<String>,
}
