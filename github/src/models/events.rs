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

#[cfg(test)]
mod test {
    use crate::models::events::RepositoryEvent;
    use crate::models::static_data::repo_events::REPO_EVENTS;
    use crate::models::SimpleUser;

    #[test]
    fn deserialize_repo_events() {
        let events: Vec<RepositoryEvent> = serde_json::from_str(REPO_EVENTS).unwrap();
        assert_eq!(events.len(), 10);
        assert_eq!(events[0].event_type, "ForkEvent");
        assert_eq!(events[1].event_type, "IssuesEvent");
        assert!(matches!(&events[1].actor, Some(SimpleUser {login, ..}) if login == "hansieodendaal"));
    }
}