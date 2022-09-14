use serde::{Deserialize, Serialize};

pub mod issue_event;
mod models;
pub mod pr_event;
pub use models::*;

use crate::error::GithubPilotError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GithubEvent {
    CommitComment(CommitCommentEvent),
    IssueComment(IssueCommentEvent),
    Issues(IssuesEvent),
    Label(LabelEvent),
    Ping(PingEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    PullRequestReviewComment(PullRequestReviewCommentEvent),
    Push(PushEvent),
    Status(StatusEvent),
    /// Catchall for as-yet unsupported event data
    UnknownEvent {
        event: String,
        payload: String,
    },
}

impl GithubEvent {
    pub fn try_from_webhook_info(event: &str, body: &str) -> Result<Self, GithubPilotError> {
        match event {
            "commit_comment" => {
                let value: CommitCommentEvent = serde_json::from_str(body)?;
                Ok(Self::CommitComment(value))
            },
            "issue_comment" => {
                let value: IssueCommentEvent = serde_json::from_str(body)?;
                Ok(Self::IssueComment(value))
            },
            "issues" => {
                let value: IssuesEvent = serde_json::from_str(body)?;
                Ok(Self::Issues(value))
            },
            "label" => {
                let value: LabelEvent = serde_json::from_str(body)?;
                Ok(Self::Label(value))
            },
            "ping" => {
                let value: PingEvent = serde_json::from_str(body)?;
                Ok(Self::Ping(value))
            },
            "pull_request" => {
                let value: PullRequestEvent = serde_json::from_str(body)?;
                Ok(Self::PullRequest(value))
            },
            "pull_request_review" => {
                let value: PullRequestReviewEvent = serde_json::from_str(body)?;
                Ok(Self::PullRequestReview(value))
            },
            "pull_request_review_comment" => {
                let value: PullRequestReviewCommentEvent = serde_json::from_str(body)?;
                Ok(Self::PullRequestReviewComment(value))
            },
            "push" => {
                let value: PushEvent = serde_json::from_str(body)?;
                Ok(Self::Push(value))
            },
            "status" => {
                let value: StatusEvent = serde_json::from_str(body)?;
                Ok(Self::Status(value))
            },
            s => Err(GithubPilotError::UnknownEvent(s.to_string())),
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::CommitComment(c) => format!("Commit comment from {}: {}", c.info.sender.login, c.comment.body),
            Self::IssueComment(c) => format!(
                "Issue comment from {}: {}",
                c.info.sender.login,
                c.comment.body.clone().unwrap_or_default()
            ),
            Self::Issues(iss) => format!("Issue {}: {}", iss.action.to_string(), iss.issue.title),
            Self::Label(lab) => format!("Label {}: {}", lab.action.to_string(), lab.label.name),
            Self::Ping(p) => format!("Ping: {}", p.zen),
            Self::PullRequest(pr) => format!("Pull request {}: {}", pr.action.to_string(), pr.pull_request.title),
            Self::PullRequestReview(r) => {
                format!("Pull request review {}: {}", r.action.to_string(), r.pull_request.title)
            },
            Self::PullRequestReviewComment(c) => {
                format!("PR review comment from {}: {}", c.info.sender.login, c.comment.body)
            },
            Self::Push(p) => format!(
                "User {} pushed {} commits to {}",
                p.pusher.name,
                p.commits.len(),
                p.info.repository.name
            ),
            Self::Status(s) => format!("Status event ({}) on {}", s.state.to_string(), s.info.repository.name),
            Self::UnknownEvent { event, .. } => format!("Unknown event: {}", event),
        }
    }

    pub fn pull_request(&self) -> Option<&PullRequestEvent> {
        match &self {
            Self::PullRequest(pr) => Some(pr),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        models::{
            static_data::events::{
                ISSUE_EVENT,
                PR_EDITED_EVENT,
                PR_EVENT,
                PR_REVIEW_COMMENT,
                PR_SYNC_EVENT,
                PUSH_EVENT,
            },
            AuthorAssociation,
            State,
        },
        webhooks::{GithubEvent, IssuesEventAction, PullRequestAction, PullRequestReviewCommentAction},
    };

    #[test]
    fn push_event() {
        let event = GithubEvent::try_from_webhook_info("push", PUSH_EVENT).unwrap();
        match event {
            GithubEvent::Push(push) => {
                assert_eq!(push.before, "455b0193f3595375025175a9f40b0552f5094437");
                assert_eq!(push.git_ref, "refs/heads/main");
                assert_eq!(push.info.repository.name, "gh-pilot");
            },
            _ => panic!("Not a push event"),
        }
    }

    #[test]
    fn pr_review_comment_event() {
        let event = GithubEvent::try_from_webhook_info("pull_request_review_comment", PR_REVIEW_COMMENT).unwrap();
        match event {
            GithubEvent::PullRequestReviewComment(c) => {
                assert!(matches!(c.action, PullRequestReviewCommentAction::Created));
                assert_eq!(c.info.sender.login, "hansieodendaal");
                assert_eq!(c.comment.id, 944298862);
                assert_eq!(c.pull_request.id, 1023688816);
                assert!(matches!(c.comment.author_association, AuthorAssociation::Collaborator));
            },
            _ => panic!("Not a pull_request_review_comment event"),
        }
    }

    #[test]
    fn pr_opened_event() {
        let event = GithubEvent::try_from_webhook_info("pull_request", PR_EVENT).unwrap();
        match event {
            GithubEvent::PullRequest(pr) => {
                assert!(matches!(pr.action, PullRequestAction::Opened));
                assert_eq!(pr.number, 2);
                assert_eq!(pr.pull_request.id, 1024763655);
                assert_eq!(
                    pr.pull_request.created_at.clone().unwrap().to_string(),
                    "2022-08-12T09:55:42Z"
                );
                assert_eq!(pr.info.repository.name, "tari-dan");
                assert_eq!(pr.info.organization.clone().unwrap().id, 37560539);
                assert_eq!(pr.info.sender.node_id, "MDQ6VXNlcjQ3OTE5OTAx");
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn pr_edited_event() {
        let event = GithubEvent::try_from_webhook_info("pull_request", PR_EDITED_EVENT).unwrap();
        match event {
            GithubEvent::PullRequest(pr) => {
                match pr.action {
                    PullRequestAction::Edited { changes } => {
                        assert!(changes.body.is_some());
                        assert_eq!(
                            changes.title.clone().unwrap().from,
                            "[wip] feat!: apply hashing api to the mmr"
                        );
                    },
                    _ => panic!("PR event action was not 'edited'"),
                }
                assert_eq!(pr.number, 4445);
                assert_eq!(
                    pr.pull_request.html_url.as_ref(),
                    "https://github.com/tari-project/tari/pull/4445"
                );
                assert_eq!(pr.info.repository.node_id, "MDEwOlJlcG9zaXRvcnkxMzY0NTkwOTk=");
                assert_eq!(
                    pr.info.organization.clone().unwrap().url,
                    "https://api.github.com/orgs/tari-project"
                );
                assert_eq!(pr.info.sender.user_type, Some("User".to_string()));
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn pr_sync_event() {
        let event = GithubEvent::try_from_webhook_info("pull_request", PR_SYNC_EVENT).unwrap();
        match event {
            GithubEvent::PullRequest(pr) => {
                match pr.action {
                    PullRequestAction::Synchronize { before, after } => {
                        assert_eq!(before, "4427dabd9c269c60ef9ebf8093d83e28d95dac82");
                        assert_eq!(after, "bdbc85470819181bf9d6243683fb7690959d6a65");
                    },
                    _ => panic!("PR event action was not 'synchronize'"),
                }
                assert_eq!(pr.number, 4445);
                assert_eq!(pr.pull_request.state, State::Open);
                assert_eq!(pr.info.repository.private, false);
                assert!(pr.info.organization.is_some());
                assert_eq!(pr.info.sender.id, 39146854);
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn issue_assigned_event() {
        let event = GithubEvent::try_from_webhook_info("issues", ISSUE_EVENT).unwrap();
        match event {
            GithubEvent::Issues(ev) => {
                match ev.action {
                    IssuesEventAction::Assigned { assignee: Some(user) } => {
                        assert_eq!(user.login, "sdbondi");
                    },
                    _ => panic!("Issue event action was not 'assigned'"),
                }
                assert_eq!(ev.issue.number, 4630);
                assert_eq!(ev.issue.id, 1364448816);
                assert_eq!(ev.issue.user.unwrap().login, "hansieodendaal");
            },
            _ => panic!("Not an issue event"),
        }
    }
}
