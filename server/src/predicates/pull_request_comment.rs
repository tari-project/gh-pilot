use github_pilot_api::{
    models::{IssueCommentEvent, PullRequestReviewCommentEvent},
    GithubEvent,
};
use serde::{Deserialize, Serialize};

use crate::{predicates::pull_request::UserName, pub_sub::GithubEventMessage, rules::RulePredicate};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestComment {
    Added(Option<UserName>),
}

impl PullRequestComment {
    pub fn from(user: &str) -> Self {
        PullRequestComment::Added(Some(user.into()))
    }

    pub fn added() -> Self {
        PullRequestComment::Added(None)
    }
}

impl RulePredicate for PullRequestComment {
    fn matches(&self, event: &GithubEventMessage) -> bool {
        use github_pilot_api::models::PullRequestReviewCommentAction::*;
        use PullRequestComment::*;
        if let GithubEvent::PullRequestReviewComment(PullRequestReviewCommentEvent { action, info, .. }) = event.event()
        {
            match (action, &self) {
                (Created, Added(Some(user))) => user.as_ref() == info.sender.login.as_str(),
                (Created, Added(None)) => true,
                _ => false,
            }
        } else if let GithubEvent::IssueComment(IssueCommentEvent { info, comment, .. }) = event.event() {
            match (comment.is_on_pull_request(), &self) {
                (true, Added(Some(user))) => user.as_ref() == info.sender.login.as_str(),
                (true, Added(None)) => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use github_pilot_api::GithubEvent;

    use crate::{
        predicates::pull_request_comment::PullRequestComment,
        pub_sub::GithubEventMessage,
        rules::RulePredicate,
    };

    #[test]
    fn new_comment_matches() {
        let data = include_str!("../../test-data/pr_review_comment_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review_comment", data).unwrap();
        let msg = GithubEventMessage::new("test", event);
        assert!(PullRequestComment::added().matches(&msg));
        assert!(!PullRequestComment::from("Bob").matches(&msg));
        assert!(PullRequestComment::from("hansieodendaal").matches(&msg));
    }

    #[test]
    fn normal_comment_matches() {
        let data = include_str!("../../test-data/issue_comment_event1.json");
        let event = GithubEvent::try_from_webhook_info("issue_comment", data).unwrap();
        let msg = GithubEventMessage::new("test", event);
        assert!(PullRequestComment::added().matches(&msg));
        assert!(!PullRequestComment::from("Bob").matches(&msg));
        assert!(PullRequestComment::from("sdbondi").matches(&msg));
    }
}
