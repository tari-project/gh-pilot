use graphql_client::GraphQLQuery;

use crate::models::DateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/pull_request_comments.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone"
)]
pub struct PullRequestComments;
