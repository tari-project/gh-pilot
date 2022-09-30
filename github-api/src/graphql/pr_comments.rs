use graphql_client::GraphQLQuery;

use crate::models::DateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/pull_request_comments.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone"
)]
pub struct PullRequestCommentsQL;

pub struct Comment {
    body: String,
    created_at: DateTime,
    author: String,
}

pub struct CommentThread {
    path: String,
    original_line: Option<i64>,
    comments: Vec<Comment>,
}

pub struct PullRequestComments {
    comments: Vec<Comment>,
    threads: Vec<CommentThread>,
}

impl From<pull_request_comments_ql::ResponseData> for PullRequestComments {
    fn from(ql: pull_request_comments_ql::ResponseData) -> Self {
        use pull_request_comments_ql::{
            PullRequestCommentsQlRepository as Repo,
            PullRequestCommentsQlRepositoryPullRequestComments as C,
            PullRequestCommentsQlRepositoryPullRequestReviewThreads as T,
            PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodes as Nodes,
            PullRequestCommentsQlRepositoryPullRequestReviewThreadsNodesComments as TC,
        };

        let (comments, thread_root) = match ql.repository {
            Some(Repo { pull_request: Some(pr) }) => (pr.comments, pr.review_threads),
            _ => (C { nodes: None }, T { nodes: None }),
        };

        let comments = comments
            .nodes
            .map(|n| {
                n.into_iter()
                    .flatten()
                    .map(|node| Comment {
                        body: node.body_text,
                        created_at: node.created_at,
                        author: node.author.map(|a| a.login).unwrap_or_default(),
                    })
                    .collect::<Vec<Comment>>()
            })
            .unwrap_or_default();

        let line = |thread: &Nodes| thread.line.or(thread.original_line).or(thread.original_start_line);

        let extract_comments = |comments: TC| {
            comments
                .nodes
                .map(|c| {
                    c.into_iter()
                        .flatten()
                        .map(|c| Comment {
                            body: c.body_text,
                            created_at: c.created_at,
                            author: c.author.map(|a| a.login).unwrap_or_default(),
                        })
                        .collect::<Vec<Comment>>()
                })
                .unwrap_or_default()
        };

        let threads = thread_root
            .nodes
            .map(|n| {
                n.into_iter()
                    .flatten()
                    .map(|thread| CommentThread {
                        original_line: line(&thread),
                        path: thread.path,
                        comments: extract_comments(thread.comments),
                    })
                    .collect::<Vec<CommentThread>>()
            })
            .unwrap_or_default();

        PullRequestComments { comments, threads }
    }
}
