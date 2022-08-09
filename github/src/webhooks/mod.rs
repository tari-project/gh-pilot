use serde::{Serialize, Deserialize};

mod models;
pub use models::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GithubEvent {
    CommitComment(models::CommitCommentEvent),
    IssueComment(models::IssueCommentEvent),
    Issues(models::IssuesEvent),
    Label(models::LabelEvent),
    Ping(models::PingEvent),
    PullRequest(models::PullRequestEvent),
    PullRequestReview(models::PullRequestReviewEvent),
    PullRequestReviewComment(models::PullRequestReviewCommentEvent),
    Push(models::PushEvent),
    Status(models::StatusEvent),
    /// Catchall for as-yet unsupported event data
    UnknownEvent{ event: String, payload: String }
}

impl GithubEvent {
    pub fn from_webhook_info(event: &str, body: &str) -> Self {
        match event {
            "commit_comment" => {
                let value: models::CommitCommentEvent = serde_json::from_str(body).unwrap();
                Self::CommitComment(value)
            }
            "issue_comment" => {
                let value: models::IssueCommentEvent = serde_json::from_str(body).unwrap();
                Self::IssueComment(value)
            }
            "issues" => {
                let value: models::IssuesEvent = serde_json::from_str(body).unwrap();
                Self::Issues(value)
            }
            "label" => {
                let value: models::LabelEvent = serde_json::from_str(body).unwrap();
                Self::Label(value)
            }
            "ping" => {
                let value: models::PingEvent = serde_json::from_str(body).unwrap();
                Self::Ping(value)
            }
            "pull_request" => {
                let value: models::PullRequestEvent = serde_json::from_str(body).unwrap();
                Self::PullRequest(value)
            }
            "pull_request_review" => {
                let value: models::PullRequestReviewEvent = serde_json::from_str(body).unwrap();
                Self::PullRequestReview(value)
            }
            "pull_request_review_comment" => {
                let value: models::PullRequestReviewCommentEvent = serde_json::from_str(body).unwrap();
                Self::PullRequestReviewComment(value)
            }
            "push" => {
                let value: models::PushEvent = serde_json::from_str(body).unwrap();
                Self::Push(value)
            }
            "status" => {
                let value: models::StatusEvent = serde_json::from_str(body).unwrap();
                Self::Status(value)
            }
            s => {
                Self::UnknownEvent { event: s.to_string(), payload: body.to_string() }
            }
        }
    }
}


