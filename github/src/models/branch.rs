use crate::models::Commit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Branch {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "commit")]
    pub commit: Commit,
    #[serde(rename = "protected")]
    pub protected: bool,
}