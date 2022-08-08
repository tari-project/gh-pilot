use crate::models::issues::Issue;
use crate::models::{Organization, RepositoryReference, SimpleUser};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Payload {
    IssueEvent(IssueEventPayload),
}

#[derive(Debug, Deserialize)]
pub struct IssueEventPayload {
    pub action: String,
    pub issue: Issue,
}

#[cfg(test)]
mod test {}
