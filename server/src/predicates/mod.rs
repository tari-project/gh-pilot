mod pull_request;
mod pull_request_comment;
mod status_checks;

use std::any::Any;

pub use pull_request::PullRequest;
pub use pull_request_comment::PullRequestComment;
use serde::{Deserialize, Serialize};
pub use status_checks::StatusCheck;

use crate::rules::RulePredicate;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Predicate {
    PullRequest(PullRequest),
    PullRequestComment(PullRequestComment),
    StatusCheck(StatusCheck),
}

impl Predicate {
    pub fn as_rule_predicate(&self) -> &dyn RulePredicate {
        match self {
            Predicate::PullRequest(pr) => pr,
            Predicate::PullRequestComment(prc) => prc,
            Predicate::StatusCheck(sc) => sc,
        }
    }
}

impl<T: RulePredicate + 'static> From<T> for Predicate {
    fn from(predicate: T) -> Self {
        if let Some(pr) = (&predicate as &dyn Any).downcast_ref::<PullRequest>() {
            Self::PullRequest(pr.clone())
        } else if let Some(prc) = (&predicate as &dyn Any).downcast_ref::<PullRequestComment>() {
            Self::PullRequestComment(prc.clone())
        } else if let Some(sc) = (&predicate as &dyn Any).downcast_ref::<StatusCheck>() {
            Self::StatusCheck(sc.clone())
        } else {
            unreachable!("Unregistered predicate type - {predicate:?}")
        }
    }
}
