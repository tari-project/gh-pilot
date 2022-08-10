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
}
