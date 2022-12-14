use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::PullRequest;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MergeParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    pub merge_method: MergeMethod,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MergeValidationError {
    pub message: String,
    pub documentation_url: String,
    #[serde(default)]
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeMethod {
    Merge,
    Rebase,
    Squash,
}

impl Default for MergeMethod {
    fn default() -> Self {
        MergeMethod::Merge
    }
}

impl FromStr for MergeMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "merge" => Ok(MergeMethod::Merge),
            "rebase" => Ok(MergeMethod::Rebase),
            "squash" => Ok(MergeMethod::Squash),
            _ => Err(format!("Invalid merge method: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MergeResult {
    pub sha: String,
    pub merged: bool,
    pub message: String,
}

impl PullRequest {
    pub fn has_merge_conflicts(&self) -> bool {
        matches!(self.mergeable, Some(false)) && !matches!(self.merged, Some(true))
    }
}
