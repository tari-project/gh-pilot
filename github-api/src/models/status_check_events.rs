use serde::{Deserialize, Serialize};

use crate::models::{App, CheckRunPullRequest, CommitSimple, CommonEventFields, DateTime, Url};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CheckSuiteAction {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "rerequested")]
    Rerequested,
}

/// The [check_suite](https://docs.github.com/en/rest/reference/checks#suites).
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "check_suite")]
pub struct CheckSuiteEvent {
    pub action: CheckSuiteAction,
    pub check_suite: CheckSuite,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

/// The [check_suite](https://docs.github.com/en/rest/reference/checks#suites).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckSuite {
    pub after: String,
    pub app: App,
    pub before: String,
    pub check_runs_url: Url,
    /// The summary conclusion for all check runs that are part of the check suite. Can be one of `success`,
    /// `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until
    /// the check run has `completed`.
    pub conclusion: Option<CheckSuiteConclusion>,
    pub created_at: DateTime,
    /// The head branch name the changes are on.
    pub head_branch: Option<String>,
    pub head_commit: CommitSimple,
    /// The SHA of the head commit that is being checked.
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    /// An array of pull requests that match this check suite. A pull request matches a check suite if they have the
    /// same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be
    /// `null` and the `pull_requests` array will be empty.
    pub pull_requests: Vec<CheckRunPullRequest>,
    /// The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or
    /// `completed`.
    pub status: Option<CheckSuiteStatus>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    /// URL that points to the check suite API resource.
    pub url: Url,
}

/// The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`,
/// `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has
/// `completed`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "stale")]
    Stale,
}

impl ToString for CheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
            Self::Failure => "failure".to_string(),
            Self::Neutral => "neutral".to_string(),
            Self::Cancelled => "cancelled".to_string(),
            Self::TimedOut => "timed_out".to_string(),
            Self::ActionRequired => "action_required".to_string(),
            Self::Skipped => "skipped".to_string(),
            Self::Stale => "stale".to_string(),
        }
    }
}
/// The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or
/// `completed`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "queued")]
    Queued,
}
impl ToString for CheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteStatus::Requested => "requested".to_string(),
            CheckSuiteStatus::InProgress => "in_progress".to_string(),
            CheckSuiteStatus::Completed => "completed".to_string(),
            CheckSuiteStatus::Queued => "queued".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        github_event::GithubEvent,
        models::status_check_events::{CheckSuiteConclusion, CheckSuiteStatus},
    };

    #[test]
    fn check_suite_event() {
        let data = include_str!("../test_data/check_suite_event.json");
        let event = GithubEvent::try_from_webhook_info("check_suite", data).unwrap();
        match event {
            GithubEvent::CheckSuiteEvent(ev) => {
                let suite = ev.check_suite;
                assert_eq!(suite.node_id, "CS_kwDOCCIzW88AAAACB8yOfw");
                assert!(matches!(suite.status, Some(CheckSuiteStatus::Completed)));
                assert!(matches!(suite.conclusion, Some(CheckSuiteConclusion::Success)));
            },
            _ => panic!("Not a check suite event"),
        }
    }
}
