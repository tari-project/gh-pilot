use serde::Deserialize;

use crate::models::{SimpleUser, State, Url};

/// Milestone : A collection of related issues and pull requests.
#[derive(Debug, Deserialize)]
pub struct Milestone {
    pub url: Url,
    pub html_url: Url,
    pub labels_url: Url,
    pub id: i32,
    pub node_id: String,
    /// The number of the milestone.
    pub number: i32,
    /// The state of the milestone.
    pub state: State,
    /// The title of the milestone.
    pub title: String,
    pub description: Option<String>,
    pub creator: Option<Box<SimpleUser>>,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub due_on: Option<String>,
}
