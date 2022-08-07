use serde::Deserialize;
use crate::models::{Label, Reactions, SimpleUser};

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub title: String,
    pub user: SimpleUser,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: bool,
    pub assignee: Option<SimpleUser>,
    pub assignees: Vec<SimpleUser>,
    pub milestone: Option<String>,
    pub comments: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub author_association: String,
    pub active_lock_reason: Option<String>,
    pub body: String,
    pub reactions: Reactions,
    pub timeline_url: String,
    pub performed_via_github_app: Option<bool>,
    pub state_reason: Option<String>,
}