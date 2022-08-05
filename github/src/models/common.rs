use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Url(String);

#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub url: Url,
    pub spdx_id: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

#[derive(Debug, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    pub maintain: bool,
    pub push: bool,
    pub triage: bool,
    pub pull: bool,
}

#[derive(Debug, Deserialize)]
pub struct Link {
    pub href: String,
}
