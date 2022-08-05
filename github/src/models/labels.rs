use crate::models::common::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Label {
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub default: bool,
}
