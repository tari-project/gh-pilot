use std::collections::{hash_map::Values, HashMap};

use graphql_client::GraphQLQuery;
use serde::Serialize;

use crate::models::{DateTime, Url};

#[allow(clippy::upper_case_acronyms)]
type URI = Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/check_run_status.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone, PartialEq, Eq"
)]
pub struct CheckRunStatusQL;

#[derive(Debug, Clone)]
pub struct CheckRunStatus {
    commit_url: Url,
    committed_at: DateTime,
    overall_status: Option<check_run_status_ql::StatusState>,
    checks: HashMap<String, RunStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum CheckResult {
    ERROR,
    FAILURE,
    PENDING,
    SUCCESS,
}

#[derive(Debug, Clone)]
pub struct RunStatus {
    pub name: String,
    pub completed_at: DateTime,
    pub result: CheckResult,
    pub is_required: bool,
}

impl Default for CheckRunStatus {
    fn default() -> Self {
        Self {
            commit_url: Url::from(""),
            committed_at: DateTime::default(),
            overall_status: None,
            checks: HashMap::new(),
        }
    }
}

use check_run_status_ql::{
    CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesCheckRunsNodes as CheckRunSuite,
    CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatusContexts as StatusContext,
};

impl CheckRunStatus {
    pub fn commit_url(&self) -> &Url {
        &self.commit_url
    }

    pub fn committed_at(&self) -> &DateTime {
        &self.committed_at
    }

    pub fn overall_status(&self) -> Option<&check_run_status_ql::StatusState> {
        self.overall_status.as_ref()
    }

    pub fn checks(&self) -> Values<String, RunStatus> {
        self.checks.values()
    }

    pub fn totals(&self) -> (usize, usize, usize) {
        let (required, passed) = self.checks().fold((0usize, 0usize), |(required, success), check| {
            if check.is_required {
                (
                    required + 1,
                    success + usize::from(check.result == CheckResult::SUCCESS),
                )
            } else {
                (required, success)
            }
        });
        (self.checks.len(), required, passed)
    }

    pub fn has_passed(&self) -> bool {
        if matches!(self.overall_status, Some(check_run_status_ql::StatusState::SUCCESS)) {
            return true;
        }
        self.checks
            .values()
            .filter(|&check| check.is_required)
            .all(|check| matches!(check.result, CheckResult::SUCCESS))
    }
}

impl From<check_run_status_ql::StatusState> for CheckResult {
    fn from(state: check_run_status_ql::StatusState) -> Self {
        match state {
            check_run_status_ql::StatusState::FAILURE => CheckResult::FAILURE,
            check_run_status_ql::StatusState::PENDING => CheckResult::PENDING,
            check_run_status_ql::StatusState::EXPECTED => CheckResult::PENDING,
            check_run_status_ql::StatusState::SUCCESS => CheckResult::SUCCESS,
            check_run_status_ql::StatusState::ERROR => CheckResult::ERROR,
            check_run_status_ql::StatusState::Other(_) => CheckResult::ERROR,
        }
    }
}

impl From<check_run_status_ql::CheckConclusionState> for CheckResult {
    fn from(state: check_run_status_ql::CheckConclusionState) -> Self {
        match state {
            check_run_status_ql::CheckConclusionState::ACTION_REQUIRED => CheckResult::FAILURE,
            check_run_status_ql::CheckConclusionState::CANCELLED => CheckResult::ERROR,
            check_run_status_ql::CheckConclusionState::FAILURE => CheckResult::FAILURE,
            check_run_status_ql::CheckConclusionState::NEUTRAL => CheckResult::FAILURE,
            check_run_status_ql::CheckConclusionState::SKIPPED => CheckResult::ERROR,
            check_run_status_ql::CheckConclusionState::STALE => CheckResult::ERROR,
            check_run_status_ql::CheckConclusionState::STARTUP_FAILURE => CheckResult::ERROR,
            check_run_status_ql::CheckConclusionState::SUCCESS => CheckResult::SUCCESS,
            check_run_status_ql::CheckConclusionState::TIMED_OUT => CheckResult::ERROR,
            check_run_status_ql::CheckConclusionState::Other(_) => CheckResult::ERROR,
        }
    }
}

impl From<CheckRunSuite> for RunStatus {
    fn from(suite: CheckRunSuite) -> Self {
        Self {
            name: suite.name,
            completed_at: suite.completed_at.unwrap_or_default(),
            result: CheckResult::from(
                suite
                    .conclusion
                    .unwrap_or(check_run_status_ql::CheckConclusionState::SKIPPED),
            ),
            is_required: suite.is_required,
        }
    }
}

impl From<StatusContext> for RunStatus {
    fn from(s: StatusContext) -> Self {
        Self {
            name: s.context,
            completed_at: s.created_at,
            result: CheckResult::from(s.state),
            is_required: s.is_required,
        }
    }
}

impl From<check_run_status_ql::ResponseData> for CheckRunStatus {
    // these damn nested structs are a nightmare to navigate. Flattening and tidying up into a nicer struct.
    fn from(res: check_run_status_ql::ResponseData) -> Self {
        use check_run_status_ql::{
            CheckRunStatusQlRepository as Repo,
            CheckRunStatusQlRepositoryPullRequest as PR,
            CheckRunStatusQlRepositoryPullRequestCommits as C,
            CheckRunStatusQlRepositoryPullRequestCommitsNodes as CN,
            CheckRunStatusQlRepositoryPullRequestCommitsNodesCommit as Commit,
        };
        let statuses = match res.repository {
            Some(Repo {
                pull_request: Some(PR {
                    commits: C { nodes: Some(v) },
                }),
            }) => v,
            _ => return CheckRunStatus::default(),
        };
        let Commit {
            url: commit_url,
            committed_date: committed_at,
            status_check_rollup,
            check_suites,
            status,
        } = match statuses.into_iter().next() {
            Some(Some(CN { commit: c })) => c,
            _ => return CheckRunStatus::default(),
        };

        let overall_status = status_check_rollup.map(|s| s.state);
        let checks = check_suites
            .into_iter()
            .flat_map(|cs| {
                cs.nodes.into_iter().flatten().flatten().flat_map(|node| {
                    node.check_runs
                        .map(|cr| cr.nodes.into_iter().flatten().flatten().map(RunStatus::from))
                })
            })
            .flatten();

        let statuses = status
            .into_iter()
            .flat_map(|s| s.contexts.into_iter().map(RunStatus::from));

        let checks = checks
            .chain(statuses)
            .fold(HashMap::new(), |mut map: HashMap<String, RunStatus>, status| {
                let insert = if let Some(existing) = map.get(&status.name) {
                    status.completed_at > existing.completed_at
                } else {
                    true
                };
                if insert {
                    map.insert(status.name.clone(), status);
                }
                map
            });
        CheckRunStatus {
            commit_url,
            committed_at,
            overall_status,
            checks,
        }
    }
}

#[cfg(test)]
mod test {
    use super::check_run_status_ql::{ResponseData, StatusState};
    use crate::graphql::{run_status::CheckResult, CheckRunStatus};

    #[test]
    fn deserialization() {
        let json = include_str!("data/sample_check_run.json");
        let res: ResponseData = serde_json::from_str(json).unwrap();
        let status = CheckRunStatus::from(res);
        assert_eq!(status.overall_status, Some(StatusState::FAILURE));
        assert_eq!(
            status.commit_url.as_ref(),
            "https://github.com/tari-project/tari/commit/eebba41dd22dbcc8ce94b86a0c25cf8d483dc3d7"
        );
        assert_eq!(status.committed_at.to_string(), "2022-10-18T11:36:33Z");
        assert_eq!(status.checks.len(), 10);

        let check = status.checks.get("check-title").unwrap();
        assert_eq!(check.name, "check-title");
        assert_eq!(check.completed_at.to_string(), "2022-10-18T15:22:48Z");
        assert!(check.is_required);
        assert_eq!(check.result, CheckResult::SUCCESS);

        let check = status.checks.get("check stable").unwrap();
        assert_eq!(check.name, "check stable");
        assert_eq!(check.result, CheckResult::SUCCESS);

        let check = status.checks.get("ci/circleci: run-ffi-integration-tests").unwrap();
        assert_eq!(check.name, "ci/circleci: run-ffi-integration-tests");
        assert_eq!(check.result, CheckResult::FAILURE);
        assert!(!check.is_required);

        let check = status.checks.get("DeepSource: Rust").unwrap();
        assert_eq!(check.name, "DeepSource: Rust");
        assert_eq!(check.result, CheckResult::SUCCESS);
        assert!(!check.is_required);
    }
}
