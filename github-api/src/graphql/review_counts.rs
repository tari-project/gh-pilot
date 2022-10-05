use graphql_client::GraphQLQuery;

use crate::{models::DateTime, wrappers::IssueId};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/pr_review_counts.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone"
)]
pub struct PullRequestReviewCountsQL;

pub struct ReviewSummary {
    pub author: String,
    pub created_at: DateTime,
    state: pull_request_review_counts_ql::PullRequestReviewState,
}

pub struct ReviewCounts {
    total_count: usize,
    id: IssueId,
    title: String,
    reviews: Vec<ReviewSummary>,
}

impl From<pull_request_review_counts_ql::ResponseData> for ReviewCounts {
    fn from(ql: pull_request_review_counts_ql::ResponseData) -> Self {
        use pull_request_review_counts_ql::{
            PullRequestReviewCountsQlRepository as Repo,
            PullRequestReviewCountsQlRepositoryPullRequest as PR,
            PullRequestReviewCountsQlRepositoryPullRequestReviews as Reviews,
        };

        let (owner, name, pr) = match ql.repository {
            Some(Repo {
                pull_request: Some(pr),
                owner,
                name,
            }) => (owner.login, name, pr),
            _ => {
                let pr = PR {
                    number: 0,
                    title: String::new(),
                    reviews: None,
                };
                (Default::default(), Default::default(), pr)
            },
        };
        let title = pr.title.clone();
        let id = IssueId::new(owner, name, pr.number as u64);
        let (total_count, nodes) = match pr.reviews {
            Some(Reviews {
                total_count,
                nodes: Some(nodes),
            }) => (total_count as usize, nodes),
            Some(Reviews {
                total_count,
                nodes: None,
            }) => (total_count as usize, vec![]),
            None => (0, vec![]),
        };
        let reviews = nodes
            .into_iter()
            .flatten()
            .map(|review| ReviewSummary {
                author: review.author.map(|a| a.login).unwrap_or_default(),
                created_at: review.created_at,
                state: review.state,
            })
            .collect::<Vec<ReviewSummary>>();

        if total_count != reviews.len() {
            log::warn!(
                "Review count mismatch: Expected {total_count}, Received {}. This is a bug requiring pagination to be \
                 added to the API query",
                reviews.len()
            );
        }

        ReviewCounts {
            total_count,
            id,
            title,
            reviews,
        }
    }
}
