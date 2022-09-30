#![allow(clippy::all, warnings)]

use crate::models::DateTime;

pub struct PullRequestCommentsQL;
pub mod pull_request_comments_ql {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "PullRequestCommentsQL";
    pub const QUERY : & str = "query PullRequestCommentsQL($owner: String!, $repo: String!, $pr_number:Int!) {\n    repository(owner:$owner, name:$repo) {\n        pullRequest(number: $pr_number) {\n            title\n            comments(last:200) {\n                edges {\n                    node {\n                        bodyText\n                        createdAt\n                        author {\n                            __typename\n                            login\n                        }\n                    }\n                }\n            }\n            reviewThreads(last: 50) {\n                nodes {\n                    line\n                    originalLine\n                    originalStartLine\n                    path\n                    resolvedBy {\n                        __typename\n                        login\n                    }\n                    comments(last: 25) {\n                        nodes {\n                            bodyText\n                            createdAt\n                            author {\n                                __typename\n                                login\n                            }\n                        }\n                    }\n                }\n            }\n        }\n    }\n}\n\n" ;
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
    type DateTime = super::DateTime;
    #[derive(Serialize)]
    pub struct Variables {
        pub owner: String,
        pub repo: String,
        pub pr_number: Int,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub repository: Option<PullRequestCommentsQlRepository>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepository {
        #[serde(rename = "pullRequest")]
        pub pull_request: Option<PullRequestCommentsQlRepositoryPullRequest>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequest {
        pub title: String,
        pub comments: PullRequestCommentsQlRepositoryPullRequestComments,
        #[serde(rename = "reviewThreads")]
        pub review_threads: PullRequestCommentsQlRepositoryPullRequestReviewThreads,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestComments {
        pub edges: Option<Vec<Option<PullRequestCommentsQlRepositoryPullRequestCommentsEdges>>>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestCommentsEdges {
        pub node: Option<PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNode {
        #[serde(rename = "bodyText")]
        pub body_text: String,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        pub author: Option<PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNodeAuthor>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNodeAuthor {
        pub login: String,
        #[serde(flatten)]
        pub on: PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNodeAuthorOn,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum PullRequestCommentsQlRepositoryPullRequestCommentsEdgesNodeAuthorOn {
        Bot,
        EnterpriseUserAccount,
        Mannequin,
        Organization,
        User,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreads {
        pub nodes: Option<Vec<Option<PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodes {
        pub line: Option<Int>,
        #[serde(rename = "originalLine")]
        pub original_line: Option<Int>,
        #[serde(rename = "originalStartLine")]
        pub original_start_line: Option<Int>,
        pub path: String,
        #[serde(rename = "resolvedBy")]
        pub resolved_by: Option<PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesResolvedBy>,
        pub comments: PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesComments,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesResolvedBy {
        pub login: String,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesComments {
        pub nodes: Option<Vec<Option<PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodes>>>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodes {
        #[serde(rename = "bodyText")]
        pub body_text: String,
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        pub author: Option<PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodesAuthor>,
    }
    #[derive(Deserialize)]
    pub struct PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodesAuthor {
        pub login: String,
        #[serde(flatten)]
        pub on: PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodesAuthorOn,
    }
    #[derive(Deserialize)]
    #[serde(tag = "__typename")]
    pub enum PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesCommentsNodesAuthorOn {
        Bot,
        EnterpriseUserAccount,
        Mannequin,
        Organization,
        User,
    }
}
impl graphql_client::GraphQLQuery for PullRequestCommentsQL {
    type ResponseData = pull_request_comments_ql::ResponseData;
    type Variables = pull_request_comments_ql::Variables;

    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: pull_request_comments_ql::QUERY,
            operation_name: pull_request_comments_ql::OPERATION_NAME,
        }
    }
}
