use crate::models::repository::Repository;
use crate::models::user::SimpleUser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitReference {
    pub label: String,
    pub r#ref: String,
    pub repo: Option<Repository>,
    pub sha: String,
    pub user: Option<SimpleUser>,
}
