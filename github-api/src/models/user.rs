use serde::{Deserialize, Serialize};

use crate::models::common::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    pub user_type: Option<UserType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum UserType {
    Bot,
    User,
    Organization,
}

impl ToString for UserType {
    fn to_string(&self) -> String {
        match *self {
            UserType::Bot => "Bot".to_string(),
            UserType::User => "User".to_string(),
            UserType::Organization => "Organization".to_string(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub site_admin: bool,
    pub contributions: i64,
    pub avatar_url: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: Option<String>,
    pub html_url: Option<String>,
    pub followers_url: Option<String>,
    pub following_url: Option<String>,
    pub gists_url: Option<String>,
    pub starred_url: Option<String>,
    pub subscriptions_url: Option<String>,
    pub organizations_url: Option<String>,
    pub repos_url: Option<String>,
    pub events_url: Option<String>,
    pub received_events_url: Option<String>,
    #[serde(rename = "type")]
    pub user_type: Option<UserType>,
}
