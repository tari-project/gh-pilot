use github_pilot_api::{
    models::{CheckSuiteConclusion, CheckSuiteStatus},
    GithubEvent,
};
use serde::{Deserialize, Serialize};

use crate::{pub_sub::GithubEventMessage, rules::RulePredicate};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StatusCheck {
    CheckSuiteSuccess,
}

impl StatusCheck {
    /// Triggers when a Status Check Suite completes successfully.
    pub fn suite_success() -> Self {
        Self::CheckSuiteSuccess
    }
}

impl RulePredicate for StatusCheck {
    fn matches(&self, event: &GithubEventMessage) -> bool {
        match (event.event(), &self) {
            (GithubEvent::CheckSuiteEvent(ev), StatusCheck::CheckSuiteSuccess) => {
                matches!(ev.check_suite.status, Some(CheckSuiteStatus::Completed)) &&
                    matches!(ev.check_suite.conclusion, Some(CheckSuiteConclusion::Success))
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::predicates::status_checks::StatusCheck;

    #[test]
    fn check_suite_success() {
        let data = include_str!("../../test-data/check_suite_event1.json");
        let event = GithubEvent::try_from_webhook_info("check_suite", data).unwrap();
        let msg = GithubEventMessage::new("check-suite", event);
        let predicate = StatusCheck::suite_success();
        assert!(predicate.matches(&msg));
    }
}
