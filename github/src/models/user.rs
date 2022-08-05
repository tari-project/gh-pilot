use crate::models::common::Url;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimpleUser {
    pub avatar_url: Url,
    pub events_url: String,
    pub followers_url: Url,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub html_url: Url,
    pub id: i64,
    pub node_id: String,
    pub login: String,
    pub organizations_url: Url,
    pub received_events_url: Url,
    pub repos_url: Url,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: Url,
    pub r#type: String,
    pub url: Url,
    pub starred_at: Option<String>,
}

pub struct User {
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
}
