#![allow(clippy::all, warnings)]
pub struct CheckRunStatusQL;
pub mod check_run_status_ql {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "CheckRunStatusQL";
    pub const QUERY : & str = "query CheckRunStatusQL($owner: String!, $repo: String!, $pr_number: Int!) {\n    repository(owner: $owner, name:$repo) {\n        pullRequest(number:$pr_number) {\n            commits(last: 1) {\n                nodes {\n                    commit {\n                        url\n                        committedDate\n                        statusCheckRollup {\n                            state\n                        }\n                        checkSuites(last: 10) {\n                            totalCount\n                            nodes {\n                                app {\n                                    name\n                                }\n                                checkRuns(last: 10) {\n                                    totalCount\n                                    nodes {\n                                        completedAt\n                                        conclusion\n                                        isRequired(pullRequestNumber: $pr_number)\n                                        name\n                                        status\n                                    }\n                                }\n                            }\n                        }\n                        status {\n                            contexts {\n                                context\n                                createdAt\n                                isRequired(pullRequestNumber: $pr_number)\n                                state\n                                description\n                            }\n                        }\n                    }\n                }\n            }\n        }\n    }\n}" ;
    use serde::{Deserialize, Serialize};

    use super::*;
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type URI = super::URI;
    type DateTime = super::DateTime;
    #[derive()]
    pub enum CheckStatusState {
        COMPLETED,
        IN_PROGRESS,
        PENDING,
        QUEUED,
        REQUESTED,
        WAITING,
        Other(String),
    }
    impl ::serde::Serialize for CheckStatusState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                CheckStatusState::COMPLETED => "COMPLETED",
                CheckStatusState::IN_PROGRESS => "IN_PROGRESS",
                CheckStatusState::PENDING => "PENDING",
                CheckStatusState::QUEUED => "QUEUED",
                CheckStatusState::REQUESTED => "REQUESTED",
                CheckStatusState::WAITING => "WAITING",
                CheckStatusState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CheckStatusState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "COMPLETED" => Ok(CheckStatusState::COMPLETED),
                "IN_PROGRESS" => Ok(CheckStatusState::IN_PROGRESS),
                "PENDING" => Ok(CheckStatusState::PENDING),
                "QUEUED" => Ok(CheckStatusState::QUEUED),
                "REQUESTED" => Ok(CheckStatusState::REQUESTED),
                "WAITING" => Ok(CheckStatusState::WAITING),
                _ => Ok(CheckStatusState::Other(s)),
            }
        }
    }
    #[derive()]
    pub enum CheckConclusionState {
        ACTION_REQUIRED,
        CANCELLED,
        FAILURE,
        NEUTRAL,
        SKIPPED,
        STALE,
        STARTUP_FAILURE,
        SUCCESS,
        TIMED_OUT,
        Other(String),
    }
    impl ::serde::Serialize for CheckConclusionState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                CheckConclusionState::ACTION_REQUIRED => "ACTION_REQUIRED",
                CheckConclusionState::CANCELLED => "CANCELLED",
                CheckConclusionState::FAILURE => "FAILURE",
                CheckConclusionState::NEUTRAL => "NEUTRAL",
                CheckConclusionState::SKIPPED => "SKIPPED",
                CheckConclusionState::STALE => "STALE",
                CheckConclusionState::STARTUP_FAILURE => "STARTUP_FAILURE",
                CheckConclusionState::SUCCESS => "SUCCESS",
                CheckConclusionState::TIMED_OUT => "TIMED_OUT",
                CheckConclusionState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for CheckConclusionState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "ACTION_REQUIRED" => Ok(CheckConclusionState::ACTION_REQUIRED),
                "CANCELLED" => Ok(CheckConclusionState::CANCELLED),
                "FAILURE" => Ok(CheckConclusionState::FAILURE),
                "NEUTRAL" => Ok(CheckConclusionState::NEUTRAL),
                "SKIPPED" => Ok(CheckConclusionState::SKIPPED),
                "STALE" => Ok(CheckConclusionState::STALE),
                "STARTUP_FAILURE" => Ok(CheckConclusionState::STARTUP_FAILURE),
                "SUCCESS" => Ok(CheckConclusionState::SUCCESS),
                "TIMED_OUT" => Ok(CheckConclusionState::TIMED_OUT),
                _ => Ok(CheckConclusionState::Other(s)),
            }
        }
    }
    #[derive()]
    pub enum StatusState {
        ERROR,
        EXPECTED,
        FAILURE,
        PENDING,
        SUCCESS,
        Other(String),
    }
    impl ::serde::Serialize for StatusState {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                StatusState::ERROR => "ERROR",
                StatusState::EXPECTED => "EXPECTED",
                StatusState::FAILURE => "FAILURE",
                StatusState::PENDING => "PENDING",
                StatusState::SUCCESS => "SUCCESS",
                StatusState::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for StatusState {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "ERROR" => Ok(StatusState::ERROR),
                "EXPECTED" => Ok(StatusState::EXPECTED),
                "FAILURE" => Ok(StatusState::FAILURE),
                "PENDING" => Ok(StatusState::PENDING),
                "SUCCESS" => Ok(StatusState::SUCCESS),
                _ => Ok(StatusState::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repo: String,
        pub pr_number: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub repository: Option<CheckRunStatusQlRepository>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepository {
        #[serde(rename = "pullRequest")]
        pub pull_request: Option<CheckRunStatusQlRepositoryPullRequest>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequest {
        pub commits: CheckRunStatusQlRepositoryPullRequestCommits,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommits {
        pub nodes: Option<Vec<Option<CheckRunStatusQlRepositoryPullRequestCommitsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodes {
        pub commit: CheckRunStatusQlRepositoryPullRequestCommitsNodesCommit,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommit {
        pub url: URI,
        #[serde(rename = "committedDate")]
        pub committed_date: DateTime,
        #[serde(rename = "statusCheckRollup")]
        pub status_check_rollup: Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatusCheckRollup>,
        #[serde(rename = "checkSuites")]
        pub check_suites: Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuites>,
        pub status: Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatus>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatusCheckRollup {
        pub state: StatusState,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuites {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub nodes: Option<Vec<Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodes {
        pub app: Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesApp>,
        #[serde(rename = "checkRuns")]
        pub check_runs: Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesCheckRuns>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesApp {
        pub name: String,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesCheckRuns {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub nodes:
            Option<Vec<Option<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesCheckRunsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitCheckSuitesNodesCheckRunsNodes {
        #[serde(rename = "completedAt")]
        pub completed_at: Option<DateTime>,
        pub conclusion: Option<CheckConclusionState>,
        #[serde(rename = "isRequired")]
        pub is_required: Boolean,
        pub name: String,
        pub status: CheckStatusState,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatus {
        pub contexts: Vec<CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatusContexts>,
    }
    #[derive(Deserialize)]
    pub struct CheckRunStatusQlRepositoryPullRequestCommitsNodesCommitStatusContexts {
        pub context: String,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "isRequired")]
        pub is_required: Boolean,
        pub state: StatusState,
        pub description: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for CheckRunStatusQL {
    type ResponseData = check_run_status_ql::ResponseData;
    type Variables = check_run_status_ql::Variables;

    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: check_run_status_ql::QUERY,
            operation_name: check_run_status_ql::OPERATION_NAME,
        }
    }
}
