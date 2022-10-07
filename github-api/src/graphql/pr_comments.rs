use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

use crate::models::DateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/pull_request_comments.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone"
)]
pub struct PullRequestCommentsQL;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comment {
    pub body: String,
    pub created_at: DateTime,
    pub author: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommentThread {
    pub path: String,
    pub original_line: Option<i64>,
    pub comments: Vec<Comment>,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct PullRequestComments {
    pub comments: Vec<Comment>,
    pub threads: Vec<CommentThread>,
}

impl PullRequestComments {
    /// Utility function to add a custom comment to the thread. Primarily used for testing.
    pub fn add_comment(&mut self, login: &str, comment: &str) -> &mut Self {
        self.comments.push(Comment {
            body: comment.to_string(),
            created_at: DateTime::now(),
            author: login.into(),
        });
        self
    }
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

#[cfg(test)]
mod test {
    use crate::graphql::PullRequestComments;

    #[test]
    fn derserialization() {
        let json = include_str!("data/sample_thread.json");
        let raw =
            serde_json::from_str::<super::pull_request_comments_ql::ResponseData>(json).expect("Failed to deserialize");
        println!("{:?}", raw);
        let threads = PullRequestComments::from(raw);
        assert_eq!(threads.comments.len(), 3);
        assert_eq!(threads.threads.len(), 2);
        assert_eq!(threads.threads[0].comments.len(), 1);
    }
}
