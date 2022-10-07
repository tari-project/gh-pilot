use graphql_client::GraphQLQuery;

use crate::{models::DateTime, wrappers::IssueId};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/pr_review_counts.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone, PartialEq, Eq"
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

impl ReviewCounts {
    pub fn total(&self) -> usize {
        self.total_count
    }

    pub fn id(&self) -> &IssueId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn approvals(&self) -> usize {
        use pull_request_review_counts_ql::PullRequestReviewState::APPROVED;
        self.reviews.iter().filter(|&r| r.state == APPROVED).count()
    }

    pub fn changes_requested(&self) -> bool {
        use pull_request_review_counts_ql::PullRequestReviewState::CHANGES_REQUESTED;
        self.reviews.iter().any(|r| r.state == CHANGES_REQUESTED)
    }

    pub fn reviewers(&self) -> Vec<String> {
        self.reviews.iter().map(|r| r.author.clone()).collect()
    }
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

#[cfg(test)]
mod test {
    use pull_request_review_counts_ql::PullRequestReviewState;

    use super::*;

    #[test]
    fn review_counts() {
        let counts = ReviewCounts {
            total_count: 3,
            id: IssueId::new("me", "proj", 12),
            title: "Foom".to_string(),
            reviews: vec![
                ReviewSummary {
                    author: "bob".to_string(),
                    state: PullRequestReviewState::APPROVED,
                    created_at: DateTime::now(),
                },
                ReviewSummary {
                    author: "andy".to_string(),
                    state: PullRequestReviewState::CHANGES_REQUESTED,
                    created_at: DateTime::now(),
                },
                ReviewSummary {
                    author: "mary".to_string(),
                    state: PullRequestReviewState::APPROVED,
                    created_at: DateTime::now(),
                },
            ],
        };

        assert_eq!(counts.total(), 3);
        assert_eq!(counts.approvals(), 2);
        assert!(counts.changes_requested());
        assert_eq!(counts.reviewers(), vec!["bob", "andy", "mary"]);
        assert_eq!(counts.id(), &IssueId::new("me", "proj", 12));
        assert_eq!(counts.title(), "Foom");
    }
}
