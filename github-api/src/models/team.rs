use serde::{Deserialize, Serialize};

use crate::models::common::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SimpleTeam {
    pub id: u64,
    pub node_id: String,
    pub url: Url,
    pub members_url: String,
    pub name: String,
    pub description: Option<String>,
    pub permission: String,
    pub privacy: Option<String>,
    pub html_url: Url,
    pub repositories_url: Url,
    pub slug: String,
    pub ldap_dn: Option<String>,
}
