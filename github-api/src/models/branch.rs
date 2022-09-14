use serde::{Deserialize, Serialize};

use crate::models::Commit;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "commit")]
    pub commit: Commit,
    #[serde(rename = "protected")]
    pub protected: bool,
}
