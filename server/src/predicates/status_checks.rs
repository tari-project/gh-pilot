use github_pilot_api::webhooks::{CheckSuiteConclusion, CheckSuiteStatus, GithubEvent};

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
