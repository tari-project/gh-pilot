use serde::{Deserialize, Serialize};

use crate::models::CommonEventFields;

//------------------------------------         Ping Event       --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEvent {
    pub hook: PingEventHook,
    pub hook_id: i64,
    pub zen: String,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

/// The [webhook configuration](https://docs.github.com/en/rest/reference/repos#get-a-repository-webhook).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHook {
    pub active: bool,
    pub config: PingEventHookConfig,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_response: Option<PingEventHookLastResponse>,
    pub name: String,
    pub ping_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHookConfig {
    pub content_type: String,
    pub insecure_ssl: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHookLastResponse {
    pub code: (),
    pub message: (),
    pub status: String,
}
