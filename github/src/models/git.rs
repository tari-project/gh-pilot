use serde::Deserialize;

use crate::models::{repository::Repository, user::SimpleUser};

#[derive(Debug, Deserialize)]
pub struct GitReference {
    pub label: String,
    pub r#ref: String,
    pub repo: Option<Repository>,
    pub sha: String,
    pub user: Option<SimpleUser>,
}
