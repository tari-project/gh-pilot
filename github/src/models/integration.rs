use crate::models::{SimpleUser, Url};
use serde::Deserialize;

/// Integration : GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub.

#[derive(Debug, Deserialize)]
pub struct Integration {
    /// Unique identifier of the GitHub app
    pub id: u64,
    /// The slug name of the GitHub app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: Option<Box<SimpleUser>>,
    /// The name of the GitHub app
    pub name: String,
    pub description: Option<String>,
    pub external_url: Url,
    pub html_url: Url,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: IntegrationPermissions,
    /// The list of events for the GitHub app
    pub events: Vec<String>,
    /// The number of installations associated with the GitHub app
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installations_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IntegrationPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<String>,
}
