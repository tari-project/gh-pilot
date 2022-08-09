use serde::{Deserialize, Serialize};

/// Installation
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationLite {
    /// The ID of the installation.
    pub id: i64,
    pub node_id: String,
}