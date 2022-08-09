use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Url(String);

impl Display for Url {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

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
