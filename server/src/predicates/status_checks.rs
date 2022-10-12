use github_pilot_api::webhook_events::{CheckSuiteConclusion, CheckSuiteStatus, GithubEvent};

use crate::{pub_sub::GithubEventMessage, rules::RulePredicate};

pub struct StatusCheck {
    trigger: StatusCheckPredicate,
}

enum StatusCheckPredicate {
    CheckSuiteSuccess,
}

impl StatusCheck {
    /// Triggers when a Status Check Suite completes successfully.
    pub fn suite_success() -> Self {
        Self {
            trigger: StatusCheckPredicate::CheckSuiteSuccess,
        }
    }
}

impl RulePredicate for StatusCheck {
    fn matches(&self, event: &GithubEventMessage) -> bool {
        match (event.event(), &self.trigger) {
            (GithubEvent::CheckSuiteEvent(ev), StatusCheckPredicate::CheckSuiteSuccess) => {
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
