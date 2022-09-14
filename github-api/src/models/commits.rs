use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    /// An array of files added in the commit.
    pub added: Vec<String>,
    pub author: Committer,
    pub committer: Committer,
    /// Whether this commit is distinct from any that have been pushed before.
    pub distinct: bool,
    pub id: String,
    /// The commit message.
    pub message: String,
    /// An array of files removed in the commit.
    pub modified: Vec<String>,
    /// An array of files modified by the commit.
    pub removed: Vec<String>,
    /// The ISO 8601 timestamp of the commit.
    pub timestamp: String,
    pub tree_id: String,
    /// URL that points to the commit API resource.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitSimple {
    pub author: Committer,
    pub committer: Committer,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Committer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    /// The git author's email address.
    pub email: String,
    /// The git author's name.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
