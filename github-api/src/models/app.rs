use serde::{Deserialize, Serialize};

use crate::models::{DateTime, SimpleUser, Url};

/// GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and
/// granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are
/// first class actors within GitHub.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub created_at: DateTime,
    pub description: Option<String>,
    /// The list of events for the GitHub app
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: Url,
    pub html_url: Url,
    /// Unique identifier of the GitHub app
    pub id: i64,
    /// The name of the GitHub app
    pub name: String,
    pub node_id: String,
    pub owner: SimpleUser,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    /// The slug name of the GitHub app
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionState {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionState {
    fn to_string(&self) -> String {
        match *self {
            Self::Read => "read".to_string(),
            Self::Write => "write".to_string(),
        }
    }
}

/// The set of permissions for the GitHub app
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discussions: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_queues: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<AppPermissionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<AppPermissionState>,
}
