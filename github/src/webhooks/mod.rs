use serde::{Deserialize, Serialize};

mod models;
pub use models::*;

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
    pub fn from_webhook_info(event: &str, body: &str) -> Self {
        match event {
            "commit_comment" => {
                let value: CommitCommentEvent = serde_json::from_str(body).unwrap();
                Self::CommitComment(value)
            },
            "issue_comment" => {
                let value: IssueCommentEvent = serde_json::from_str(body).unwrap();
                Self::IssueComment(value)
            },
            "issues" => {
                let value: IssuesEvent = serde_json::from_str(body).unwrap();
                Self::Issues(value)
            },
            "label" => {
                let value: LabelEvent = serde_json::from_str(body).unwrap();
                Self::Label(value)
            },
            "ping" => {
                let value: PingEvent = serde_json::from_str(body).unwrap();
                Self::Ping(value)
            },
            "pull_request" => {
                let value: PullRequestEvent = serde_json::from_str(body).unwrap();
                Self::PullRequest(value)
            },
            "pull_request_review" => {
                let value: PullRequestReviewEvent = serde_json::from_str(body).unwrap();
                Self::PullRequestReview(value)
            },
            "pull_request_review_comment" => {
                let value: PullRequestReviewCommentEvent = serde_json::from_str(body).unwrap();
                Self::PullRequestReviewComment(value)
            },
            "push" => {
                let value: PushEvent = serde_json::from_str(body).unwrap();
                Self::Push(value)
            },
            "status" => {
                let value: StatusEvent = serde_json::from_str(body).unwrap();
                Self::Status(value)
            },
            s => Self::UnknownEvent {
                event: s.to_string(),
                payload: body.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{models::static_data::events::PUSH_EVENT, webhooks::GithubEvent};
    use crate::models::{AuthorAssociation, State};
    use crate::models::static_data::events::{PR_EDITED_EVENT, PR_EVENT, PR_REVIEW_COMMENT, PR_SYNC_EVENT};
    use crate::webhooks::{PullRequestAction, PullRequestReviewCommentAction};

    #[test]
    fn push_event() {
        let event = GithubEvent::from_webhook_info("push", PUSH_EVENT);
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
        let event = GithubEvent::from_webhook_info("pull_request_review_comment", PR_REVIEW_COMMENT);
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
        let event = GithubEvent::from_webhook_info("pull_request", PR_EVENT);
        match event {
            GithubEvent::PullRequest(pr) => {
                assert!(matches!(pr.action, PullRequestAction::Opened));
                assert_eq!(pr.number, 2);
                assert_eq!(pr.pull_request.id, 1024763655);
                assert_eq!(pr.pull_request.created_at.clone().unwrap().to_string(), "2022-08-12T09:55:42Z");
                assert_eq!(pr.info.repository.name, "tari-dan");
                assert_eq!(pr.info.organization.clone().unwrap().id, 37560539);
                assert_eq!(pr.info.sender.node_id, "MDQ6VXNlcjQ3OTE5OTAx");
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn pr_edited_event() {
        let event = GithubEvent::from_webhook_info("pull_request", PR_EDITED_EVENT);
        match event {
            GithubEvent::PullRequest(pr) => {
                match pr.action {
                    PullRequestAction::Edited {changes} => {
                        assert!(changes.body.is_some());
                        assert_eq!(changes.title.clone().unwrap().from, "[wip] feat!: apply hashing api to the mmr");
                    },
                    _ => panic!("PR event action was not 'edited'"),
                }
                assert_eq!(pr.number, 4445);
                assert_eq!(pr.pull_request.html_url.as_ref(), "https://github.com/tari-project/tari/pull/4445");
                assert_eq!(pr.info.repository.node_id, "MDEwOlJlcG9zaXRvcnkxMzY0NTkwOTk=");
                assert_eq!(pr.info.organization.clone().unwrap().url, "https://api.github.com/orgs/tari-project");
                assert_eq!(pr.info.sender.user_type, Some("User".to_string()));
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn pr_sync_event() {
        let event = GithubEvent::from_webhook_info("pull_request", PR_SYNC_EVENT);
        match event {
            GithubEvent::PullRequest(pr) => {
                match pr.action {
                    PullRequestAction::Synchronize {before, after} => {
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
}
