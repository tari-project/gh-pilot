use serde::{ Deserialize, Serialize };

use crate::models::{repository::Repository, user::SimpleUser};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GitReference {
    pub label: String,
    pub r#ref: String,
    pub repo: Option<Repository>,
    pub sha: String,
    pub user: Option<SimpleUser>,
}
