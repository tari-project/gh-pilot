use crate::models::issues::Issue;
use serde::Deserialize;
use crate::models::{Organization, RepositoryReference, SimpleUser};

#[derive(Debug, Deserialize)]
pub enum Payload {
    IssueEvent(IssueEventPayload)
}

#[derive(Debug, Deserialize)]
pub struct IssueEventPayload {
    pub action: String,
    pub issue: Issue,
}


#[derive(Debug, Deserialize)]
pub struct RepositoryEvent {
    pub id: String,
    #[serde(rename="type")]
    pub event_type: String,
    pub actor: Option<SimpleUser>,
    pub repo: RepositoryReference,
    pub payload: Payload,
    pub public: bool,
    pub created_at: String,
    pub org: Organization,
}