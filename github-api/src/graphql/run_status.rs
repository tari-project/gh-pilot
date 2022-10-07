use std::slice::Iter;

use graphql_client::GraphQLQuery;
use log::warn;

use crate::models::{DateTime, Url};

type URI = Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/check_run_status.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone, PartialEq, Eq"
)]
pub struct CheckRunStatusQL;

pub struct CheckRunStatus {
    commit_url: Url,
    committed_at: DateTime,
    overall_status: Option<check_run_status_ql::StatusState>,
    checks: Vec<RunStatus>,
}

pub struct RunStatus {
    pub name: String,
    pub completed_at: DateTime,
    pub result: check_run_status_ql::CheckConclusionState,
    pub status: check_run_status_ql::CheckStatusState,
    pub is_required: bool,
}

impl Default for CheckRunStatus {
    fn default() -> Self {
        Self {
            commit_url: Url::from(""),
            committed_at: DateTime::default(),
            overall_status: None,
            checks: vec![],
        }
    }
}

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

    pub fn checks(&self) -> Iter<RunStatus> {
        self.checks.iter()
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
        } = match statuses.into_iter().next() {
            Some(Some(CN { commit: c })) => c,
            _ => return CheckRunStatus::default(),
        };
        let overall_status = status_check_rollup.map(|s| s.state);
        let checks = match check_suites {
            Some(cs) => {
                let checks = cs
                    .nodes
                    .into_iter()
                    .flatten()
                    .flatten()
                    .flat_map(|node| {
                        let app_name = node
                            .app
                            .map(|app| app.name)
                            .unwrap_or_else(|| "App name not provided".to_string());
                        match node.check_runs {
                            Some(cr) => {
                                let run_count = cr.total_count as usize;
                                let fetched = cr.nodes.iter().flatten().flatten().count();
                                if run_count != fetched {
                                    warn!(
                                        "There are {run_count} check runs for {app_name}, but only {fetched} were \
                                         fetched. File a bug report saying we need proper pagination in the \
                                         run_status query code."
                                    );
                                }
                                cr.nodes
                                    .into_iter()
                                    .flatten()
                                    .flatten()
                                    .map(|run| {
                                        let name = run.name;
                                        let completed_at = run.completed_at.unwrap_or_default();
                                        let result = run.conclusion.unwrap_or_else(|| {
                                            check_run_status_ql::CheckConclusionState::Other("unknown".to_string())
                                        });
                                        let status = run.status;
                                        let is_required = run.is_required;
                                        RunStatus {
                                            name,
                                            completed_at,
                                            result,
                                            status,
                                            is_required,
                                        }
                                    })
                                    .collect::<Vec<RunStatus>>()
                            },
                            None => vec![],
                        }
                    })
                    .collect();
                checks
            },
            None => vec![],
        };

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
    use super::check_run_status_ql::{CheckConclusionState, CheckStatusState, ResponseData, StatusState};
    use crate::graphql::CheckRunStatus;

    #[test]
    fn deserialization() {
        let json = include_str!("data/sample_check_run.json");
        let res: ResponseData = serde_json::from_str(json).unwrap();
        let status = CheckRunStatus::from(res);
        assert_eq!(status.overall_status, Some(StatusState::SUCCESS));
        assert_eq!(
            status.commit_url.as_ref(),
            "https://github.com/tari-project/gh-pilot/commit/c13501c51f6a3f5183f5921aa89d68110a29f0f4"
        );
        assert_eq!(status.committed_at.to_string(), "2022-09-20T15:30:05Z");
        assert_eq!(status.checks.len(), 5);
        let check = &status.checks[0];
        assert_eq!(check.name, "clippy_check");
        assert_eq!(check.completed_at.to_string(), "2022-09-20T15:35:54Z");
        assert!(!check.is_required);
        assert_eq!(check.result, CheckConclusionState::SUCCESS);
        assert_eq!(check.status, CheckStatusState::COMPLETED);
        let check = &status.checks[1];
        assert_eq!(check.name, "check-title");
        assert_eq!(check.result, CheckConclusionState::SUCCESS);
        assert_eq!(check.status, CheckStatusState::COMPLETED);
        let check = &status.checks[2];
        assert_eq!(check.name, "cargo test (stable)");
        let check = &status.checks[3];
        assert_eq!(check.name, "cargo test (nightly-2022-09-13)");
        let check = &status.checks[4];
        assert_eq!(check.name, "check-title");
    }
}
