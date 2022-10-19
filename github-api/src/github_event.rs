use serde::{Deserialize, Serialize};

use crate::{
    error::GithubProviderError,
    models::{
        CheckSuiteEvent,
        CommitCommentEvent,
        IssueCommentEvent,
        IssuesEvent,
        LabelEvent,
        PingEvent,
        PullRequestEvent,
        PullRequestReviewCommentEvent,
        PullRequestReviewEvent,
        PushEvent,
        StatusEvent,
    },
    wrappers::IssueId,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GithubEvent {
    CheckSuiteEvent(CheckSuiteEvent),
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
    pub fn try_from_webhook_info(event: &str, body: &str) -> Result<Self, GithubProviderError> {
        match event {
            "check_suite" => {
                let event: CheckSuiteEvent = serde_json::from_str(body)?;
                Ok(GithubEvent::CheckSuiteEvent(event))
            },
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
            s => Err(GithubProviderError::UnknownEvent(s.to_string())),
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::CheckSuiteEvent(e) => e.check_suite.summary(),
            Self::CommitComment(c) => format!("Commit comment from {}: {}", c.info.sender.login, c.comment.body),
            Self::IssueComment(c) => format!(
                "Issue comment from {}: {}",
                c.info.sender.login,
                c.comment.body.clone().unwrap_or_default()
            ),
            Self::Issues(iss) => format!("Issue {}: {}", iss.action.to_string(), iss.issue.title),
            Self::Label(lab) => format!("Label {}: {}", lab.action.to_string(), lab.label.name),
            Self::Ping(p) => format!("Ping: {}", p.zen),
            Self::PullRequest(pr) => format!("Pull request {}: {}", pr.action, pr.pull_request.title),
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

    /// If this event is related to a pull request, this method conveniently returns the PR details as an [`IssueId`].
    pub fn related_pull_request(&self) -> Option<IssueId> {
        match &self {
            GithubEvent::CheckSuiteEvent(ev) => ev.first_related_pr(),
            GithubEvent::PullRequest(ev) => Some(ev.as_issue_id()),
            GithubEvent::PullRequestReview(ev) => Some(ev.related_pull_request()),
            GithubEvent::PullRequestReviewComment(ev) => Some(ev.related_pull_request()),
            GithubEvent::Push(_) => None,
            GithubEvent::Status(_) => None,
            GithubEvent::Label(_) => None,
            GithubEvent::CommitComment(_) => None,
            GithubEvent::IssueComment(ev) => ev.related_pull_request(),
            GithubEvent::Issues(_) => None,
            GithubEvent::Ping(_) => None,
            GithubEvent::UnknownEvent { .. } => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        github_event::GithubEvent,
        models::{
            AuthorAssociation,
            IssueCommentAction,
            IssuesEventAction,
            PullRequestAction,
            PullRequestReviewAction,
            PullRequestReviewCommentAction,
            ReviewState,
            State,
            UserType,
        },
    };

    #[test]
    fn push_event() {
        let data = include_str!("test_data/push_event.json");
        let event = GithubEvent::try_from_webhook_info("push", data).unwrap();
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
    fn push_event_2() {
        let data = include_str!("test_data/push_event2.json");
        let event = GithubEvent::try_from_webhook_info("push", data).unwrap();
        match event {
            GithubEvent::Push(push) => {
                assert_eq!(push.info.repository.name, "gh-pilot");
            },
            _ => panic!("Not a push event"),
        }
    }

    #[test]
    fn labelled_event() {
        let data = include_str!("test_data/labelled_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request", data).unwrap();
        let pr = event.pull_request().expect("Labelled PR event did not include the PR");
        assert_eq!(pr.pull_request.state, State::Open);
        assert_eq!(pr.number, 2);
        match &pr.action {
            PullRequestAction::Labeled { label } => {
                assert_eq!(label.name, "T-foo");
                assert_eq!(label.color, "7A2F2E");
            },
            _ => panic!("Action was not 'label'"),
        }
    }

    #[test]
    fn pr_review_comment_event() {
        let data = include_str!("test_data/pr_review_comment_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review_comment", data).unwrap();
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
        let data = include_str!("test_data/pr_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request", data).unwrap();
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
        let data = include_str!("test_data/pr_edited_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request", data).unwrap();
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
                assert_eq!(pr.info.sender.user_type, Some(UserType::User));
            },
            _ => panic!("Not a pull_request event"),
        }
    }

    #[test]
    fn pr_sync_event() {
        let data = include_str!("test_data/pr_sync_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request", data).unwrap();
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
        let data = include_str!("test_data/issue_event.json");
        let event = GithubEvent::try_from_webhook_info("issues", data).unwrap();
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

    /// Fix bug in deserializing the `GithubEvent::PullRequestReviewComment` event - 15/9/2022
    #[test]
    fn pr_review_comment() {
        let data = include_str!("test_data/pr_review_comment_event2.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review_comment", data).unwrap();
        match event {
            GithubEvent::PullRequestReviewComment(ev) => {
                assert!(matches!(ev.action, PullRequestReviewCommentAction::Created));
                assert_eq!(ev.comment.node_id, "PRRC_kwDOCCIzW845_Sdp");
                assert_eq!(ev.comment.user.login, "sdbondi");
            },
            _ => panic!("Not a PullRequestReviewComment event"),
        }
    }

    /// Fix bug in deserializing the `GithubEvent::IssueComment` event - 15/9/2022
    #[test]
    fn issue_comment_1() {
        let data = include_str!("test_data/issue_comment1.json");
        let event = GithubEvent::try_from_webhook_info("issue_comment", data).unwrap();
        match event {
            GithubEvent::IssueComment(ev) => {
                assert!(matches!(ev.action, IssueCommentAction::Created));
                assert_eq!(ev.issue.id, 1375932113);
                assert_eq!(ev.comment.id, 1249350083);
                assert_eq!(ev.comment.user.unwrap().login, "CjS77");
            },
            _ => panic!("Not a PullRequestReviewComment event"),
        }
    }

    /// Fix bug in deserializing the `GithubEvent::PullRequestReview` event - 19/9/2022
    #[test]
    fn pr_review_1() {
        let data = include_str!("test_data/pull_request_review.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review", data).unwrap();
        match event {
            GithubEvent::PullRequestReview(ev) => match ev.action {
                PullRequestReviewAction::Edited { changes } => {
                    assert!(changes.body.is_none());
                    assert_eq!(ev.pull_request.id, 1060040769);
                    assert!(ev.review.body.starts_with("I agree with the added safeguard"));
                },
                _ => panic!("PR review event action was not 'edited'"),
            },
            _ => panic!("Not a PullRequestReviewComment event"),
        }
    }

    #[test]
    fn pr_review_commented() {
        let data = include_str!("test_data/pr_review_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review", data).unwrap();
        match event {
            GithubEvent::PullRequestReview(ev) => {
                assert!(matches!(ev.action, PullRequestReviewAction::Submitted));
                assert_eq!(ev.info.sender.login, "jorgeantonio21");
                let review = ev.review;
                assert!(matches!(review.state, ReviewState::Commented));
                assert_eq!(review.node_id, "PRR_kwDOCCIzW85DsRe7");
                assert_eq!(review.commit_id, "6ec69020efbb56752797f05f86cde07704e1d256");
            },
            _ => panic!("Not a PullRequestReview event"),
        }
    }

    // Regression test for issue comment deserialization bug.
    #[test]
    fn issue_comment_event() {
        let data = include_str!("test_data/issue_comment_event1.json");
        let event = GithubEvent::try_from_webhook_info("issue_comment", data).unwrap();
        match event {
            GithubEvent::IssueComment(ev) => {
                if let IssueCommentAction::Edited { changes, comment } = ev.action {
                    assert!(matches!(changes.body, Some(c) if c.from == "ACK"));
                    assert!(comment.is_none());
                } else {
                    panic!("Issue comment event action was not 'edited'");
                }
            },
            _ => panic!("Not an Issue Comment event"),
        }
    }

    #[test]
    fn pull_request_comment() {
        let data = include_str!("test_data/pr_review_comment3.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review_comment", data).unwrap();
        match event {
            GithubEvent::PullRequestReviewComment(ev) => {
                if let PullRequestReviewCommentAction::Edited(p) = ev.action {
                    let change = (p.body.as_ref()).unwrap();
                    assert!(change.from.starts_with("Looks like upserts"));
                } else {
                    panic!("pull_request_review_comment event action was not 'edited'");
                }
                assert_eq!(ev.comment.id, 999041536);
                assert_eq!(ev.pull_request.number, 104);
            },
            _ => panic!("Not an pull_request_review_comment event"),
        }
    }
}
