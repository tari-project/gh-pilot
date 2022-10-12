use serde::{Deserialize, Serialize};

use crate::models::{repository::Repository, user::SimpleUser, RepositoryReference};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitReference {
    pub label: String,
    pub r#ref: String,
    pub repo: Option<Repository>,
    pub sha: String,
    pub user: Option<SimpleUser>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitReferenceShort {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: RepositoryReference,
    pub sha: String,
}
