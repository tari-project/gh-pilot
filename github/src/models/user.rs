use crate::models::common::Url;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SimpleUser {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: Url,
    pub url: Url,
    pub events_url: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub gravatar_id: Option<String>,
    pub starred_at: Option<String>,
    pub followers_url: Option<Url>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub html_url: Option<Url>,
    pub organizations_url: Option<Url>,
    pub received_events_url: Option<Url>,
    pub repos_url: Option<Url>,
    pub site_admin: Option<bool>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<Url>,
    #[serde(rename = "type")]
    pub user_type: Option<String>,
}
