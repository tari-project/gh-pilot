use serde::{Deserialize, Serialize};

use crate::models::{DateTime, Issue, IssueComment, Organization, PullRequest, RepositoryReference, ShortCommit, Url};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(flatten)]
    info: CommonEventInfo,
    #[serde(flatten)]
    event: EventPayload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum EventPayload {
    CreateEvent(CreateEventPayload),
    DeleteEvent(DeleteEventPayload),
    IssueCommentEvent(IssueCommentEventPayload),
    IssuesEvent(IssuesEventPayload),
    PullRequestEvent(PullRequestEventPayload),
    PushEvent(PushEventPayload),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonEventInfo {
    pub id: String,
    pub actor: Organization,
    #[serde(rename = "repo")]
    pub repo: RepositoryReference,
    #[serde(rename = "org", skip_serializing_if = "Option::is_none")]
    pub org: Option<Organization>,
    pub public: bool,
    pub created_at: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushEventPayload {
    pub push_id: u64,
    pub size: u64,
    pub distinct_size: u64,
    #[serde(rename = "ref")]
    pub url: Url,
    pub head: String,
    pub before: String,
    pub commits: Vec<ShortCommit>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEventPayload {
    pub action: String,
    pub number: u64,
    pub pull_request: PullRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateEventPayload {
    #[serde(rename = "ref")]
    pub reference: String,
    pub ref_type: String,
    pub master_branch: String,
    pub description: String,
    pub pusher_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeleteEventPayload {
    #[serde(rename = "ref")]
    pub reference: String,
    pub ref_type: String,
    pub pusher_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEventPayload {
    pub action: String,
    pub issue: Issue,
    pub comment: IssueComment,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEventPayload {
    pub action: String,
    pub issue: Issue,
}

#[cfg(test)]
mod test {
    use crate::models::{Event, EventPayload};

    #[test]
    fn events() {
        let data = include_str!("../test_data/events.json");
        let events: Vec<Event> = serde_json::from_str(data).unwrap();
        assert_eq!(events.len(), 15);
        let ev = &events[0];
        assert_eq!(ev.info.actor.login, "CjS77");
        assert_eq!(ev.info.repo.name, "tari-project/gh-pilot");
        if let EventPayload::PushEvent(push) = &ev.event {
            assert_eq!(push.push_id, 12209393475);
            assert_eq!(push.commits.len(), 1);
            assert_eq!(push.commits[0].sha, "37fc4c72643f5c87271055cc7f6b9f9e09d276dc");
        } else {
            panic!("event 0 was not a push event");
        }

        let ev = &events[1];
        assert_eq!(ev.info.id, "26283068609");
        if let EventPayload::PullRequestEvent(pr) = &ev.event {
            assert_eq!(pr.number, 70);
            assert_eq!(pr.action, "opened");
            assert_eq!(pr.pull_request.commits, Some(1));
        } else {
            panic!("event 1 was not a pull request event");
        }

        let ev = &events[5];
        assert_eq!(ev.info.id, "26282179452");
        if let EventPayload::IssueCommentEvent(ic) = &ev.event {
            assert_eq!(ic.action, "created");
            assert_eq!(ic.issue.number, 69);
            assert_eq!(ic.comment.body, Some("ACK".into()));
        } else {
            panic!("event 5 was not an issue comment event");
        }

        let ev = &events[6];
        assert_eq!(ev.info.id, "26273148166");
        if let EventPayload::IssuesEvent(ie) = &ev.event {
            assert_eq!(ie.action, "closed");
            assert_eq!(ie.issue.number, 45);
        } else {
            panic!("event 6 was not an issues event");
        }
    }
}
