use std::{iter, path::PathBuf};

use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};

use crate::models::{DateTime, Url};

#[allow(clippy::upper_case_acronyms)]
type URI = Url;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/data/schema.graphql",
    query_path = "src/graphql/data/org_activity.graphql",
    deprecated = "warn",
    response_derives = "Debug, Clone"
)]
pub struct OrgActivityQL;

use crate::graphql::org_activity::org_activity_ql::{
    issueFields,
    pageInfoFields,
    prFields,
    CommentFieldsAuthor,
    IssueFieldsAuthor,
    IssueFieldsComments,
    OrgActivityQlSearch,
    OrgActivityQlSearchEdgesNode,
    PrFieldsAuthor,
    PrFieldsComments,
    PrFieldsFiles,
    PrFieldsReviews,
    PrFieldsReviewsNodesAuthor,
    ResponseData,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct OrgActivity {
    pub issues: Vec<IssueActivity>,
    pub pull_requests: Vec<PullRequestActivity>,
}

impl OrgActivity {
    pub fn merge(&mut self, other: OrgActivity) {
        self.issues.extend(other.issues);
        self.pull_requests.extend(other.pull_requests);
    }
}

/// A user field. To [Self::display_name] to return the user's name, or if that is missing, their login
#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct User {
    pub name: Option<String>,
    pub login: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: Some("Anonymous".to_string()),
            login: "anon".to_string(),
        }
    }
}

impl User {
    pub fn new(login: String, name: Option<String>) -> Self {
        Self { login, name }
    }

    pub fn display_name(&self) -> &str {
        self.name.as_ref().unwrap_or(&self.login).as_str()
    }

    pub fn login_with_name(&self) -> String {
        match &self.name {
            Some(n) => format!("{} ({n})", self.login),
            None => self.login.to_string(),
        }
    }
}

impl ToString for User {
    fn to_string(&self) -> String {
        self.display_name().to_string()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PullRequestActivity {
    pub number: u64,
    pub title: String,
    pub base_repository: Option<String>,
    pub author: User,
    pub created_at: DateTime,
    pub changed_file_count: u64,
    pub total_deletions: u64,
    pub total_additions: u64,
    pub merged: bool,
    pub closed: bool,
    pub comments: CommentSummary,
    pub files: FilesChangedSummary,
    pub reviews: ReviewsSummary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssueActivity {
    pub author: User,
    pub title: String,
    pub number: u64,
    pub url: Url,
    pub comments: CommentSummary,
    pub body: String,
    pub closed: bool,
    pub created_at: DateTime,
    pub repository: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommentSummary {
    pub total_comments: u64,
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub author: User,
    pub text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FilesChangedSummary {
    pub files_changed: Vec<FileChange>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileChange {
    pub path: PathBuf,
    pub additions: u64,
    pub deletions: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReviewsSummary {
    pub reviews: Vec<Review>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    pub total_comments: u64,
    pub author: User,
}

#[derive(Clone, Debug)]
pub struct OrgActivitySearch {
    pub page_info: pageInfoFields,
    pub total_count: u64,
    pub search_result: OrgActivity,
}

impl From<ResponseData> for OrgActivitySearch {
    fn from(value: ResponseData) -> Self {
        let search_results = value.search;
        let OrgActivityQlSearch {
            page_info,
            edges,
            issue_count,
        } = search_results;
        let result_nodes = edges
            .map(|edge_vec| {
                let iter = edge_vec.into_iter().flatten().filter_map(|v| v.node);
                Box::new(iter) as Box<dyn Iterator<Item = OrgActivityQlSearchEdgesNode>>
            })
            .unwrap_or(Box::new(iter::empty::<OrgActivityQlSearchEdgesNode>())
                as Box<dyn Iterator<Item = OrgActivityQlSearchEdgesNode>>);
        let search_result = OrgActivity::from_iter(result_nodes);
        OrgActivitySearch {
            page_info,
            total_count: issue_count as u64,
            search_result,
        }
    }
}

impl FromIterator<OrgActivityQlSearchEdgesNode> for OrgActivity {
    fn from_iter<T: IntoIterator<Item = OrgActivityQlSearchEdgesNode>>(iter: T) -> Self {
        use OrgActivityQlSearchEdgesNode::*;
        let (issues, pull_requests): (Vec<_>, Vec<_>) = iter
            .into_iter()
            .filter(|n| matches!(n, Issue(_) | PullRequest(_)))
            .partition(|n| matches!(n, Issue(_)));
        let issues = issues
            .into_iter()
            .map(|i| match i {
                Issue(is) => IssueActivity::from(is),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let pull_requests = pull_requests
            .into_iter()
            .map(|i| match i {
                PullRequest(pr) => PullRequestActivity::from(pr),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        Self { issues, pull_requests }
    }
}

impl From<issueFields> for IssueActivity {
    fn from(value: issueFields) -> Self {
        Self {
            author: value.author.map(User::from).unwrap_or_default(),
            title: value.title,
            number: value.number as u64,
            url: value.url,
            comments: CommentSummary::from(value.comments),
            body: value.body_text,
            closed: value.closed,
            created_at: value.created_at,
            repository: value.repository.name,
        }
    }
}

impl From<prFields> for PullRequestActivity {
    fn from(value: prFields) -> Self {
        Self {
            number: value.number as u64,
            title: value.title,
            base_repository: value.base_repository.map(|b| b.name),
            author: value.author.map(User::from).unwrap_or_default(),
            created_at: value.created_at,
            changed_file_count: value.changed_files as u64,
            total_deletions: value.deletions as u64,
            total_additions: value.additions as u64,
            merged: value.merged,
            closed: value.closed,
            comments: CommentSummary::from(value.comments),
            files: value.files.map(FilesChangedSummary::from).unwrap_or_default(),
            reviews: value.reviews.map(ReviewsSummary::from).unwrap_or_default(),
        }
    }
}

impl From<IssueFieldsComments> for CommentSummary {
    fn from(value: IssueFieldsComments) -> Self {
        let total_comments = value.total_count as u64;
        let comments = value
            .nodes
            .into_iter()
            .flatten()
            .flatten()
            .map(|n| Comment {
                author: n.author.map(User::from).unwrap_or_default(),
                text: n.body_text,
            })
            .collect::<Vec<_>>();
        Self {
            total_comments,
            comments,
        }
    }
}

impl From<PrFieldsComments> for CommentSummary {
    fn from(value: PrFieldsComments) -> Self {
        let total_comments = value.total_count as u64;
        let comments = value
            .nodes
            .into_iter()
            .flatten()
            .flatten()
            .map(|pr| Comment {
                author: pr.author.map(User::from).unwrap_or_default(),
                text: pr.body_text,
            })
            .collect::<Vec<_>>();
        Self {
            total_comments,
            comments,
        }
    }
}

impl From<PrFieldsFiles> for FilesChangedSummary {
    fn from(value: PrFieldsFiles) -> Self {
        let files_changed = value
            .nodes
            .into_iter()
            .flatten()
            .flatten()
            .map(|n| FileChange {
                path: PathBuf::from(n.path),
                additions: n.additions as u64,
                deletions: n.deletions as u64,
            })
            .collect();
        Self { files_changed }
    }
}

impl From<PrFieldsReviews> for ReviewsSummary {
    fn from(value: PrFieldsReviews) -> Self {
        let reviews = value
            .nodes
            .into_iter()
            .flatten()
            .flatten()
            .map(|n| Review {
                total_comments: n.comments.total_count as u64,
                author: n.author.map(User::from).unwrap_or_default(),
            })
            .collect();
        Self { reviews }
    }
}

macro_rules! toUser {
    ($ty:ident) => {
        impl From<$ty> for User {
            fn from(value: $ty) -> Self {
                match value {
                    $ty::Bot => User::new("Bot".to_string(), None),
                    $ty::EnterpriseUserAccount => User::new("EnterpriseUserAccount".to_string(), None),
                    $ty::Mannequin => User::new("Mannequin".to_string(), None),
                    $ty::Organization => User::new("Organization".to_string(), None),
                    $ty::User(u) => User::new(u.login, u.name),
                }
            }
        }
    };
}

toUser!(IssueFieldsAuthor);
toUser!(PrFieldsAuthor);
toUser!(CommentFieldsAuthor);
toUser!(PrFieldsReviewsNodesAuthor);

#[cfg(test)]
mod test {
    use crate::graphql::org_activity::{org_activity_ql::ResponseData, OrgActivitySearch};

    #[test]
    fn org_activity_deserialization_and_conversion() {
        let json = include_str!("data/sample_activity.json");
        let raw = serde_json::from_str::<ResponseData>(json).expect("Failed to deserialize");
        let res = OrgActivitySearch::from(raw);
        assert_eq!(res.page_info.start_cursor, Some("Y3Vyc29yOjE2".into()));
        assert_eq!(res.page_info.end_cursor, Some("Y3Vyc29yOjQw".into()));
        assert!(res.page_info.has_next_page);
        assert!(res.page_info.has_previous_page);
        assert_eq!(res.total_count, 355);
        let results = res.search_result;
        assert_eq!(results.issues.len(), 6);
        assert_eq!(results.pull_requests.len(), 19);
        let issue = &results.issues[2];
        assert_eq!(issue.author.display_name(), "SW van Heerden");
        assert!(issue.closed);
        assert_eq!(issue.comments.total_comments, 0);
        assert_eq!(
            issue.title,
            "Base_node sync, should remove blocks from orphan db it syncs and not leave them in the orphan db"
        );
        assert_eq!(issue.number, 4867);
        assert_eq!(issue.repository, "tari");
        assert_eq!(issue.url.as_ref(), "https://github.com/tari-project/tari/issues/4867");
        assert!(issue.body.starts_with("base-node.zip\nKinda linked to #4866"));
        assert_eq!(issue.created_at.to_string(), "2022-10-28T14:17:05Z");

        let pr = &results.pull_requests[18];
        assert_eq!(pr.created_at.to_string(), "2022-10-26T20:25:33Z");
        assert_eq!(pr.comments.total_comments, 1);
        assert_eq!(pr.comments.comments[0].text, "ACK");
        assert_eq!(pr.number, 36);
        assert!(pr.closed);
        assert!(pr.merged);
        assert_eq!(pr.title, "feat: stabilise rfc-0173");
        assert_eq!(pr.author.login, "CjS77");
        assert_eq!(pr.reviews.reviews.len(), 4);
        assert_eq!(pr.base_repository.as_ref().unwrap(), "rfcs");
        assert_eq!(pr.changed_file_count, 1);
        assert_eq!(pr.total_additions, 34);
        assert_eq!(pr.total_deletions, 14);
        let f = &pr.files.files_changed[0];
        assert_eq!(f.additions, 34);
        assert_eq!(f.deletions, 14);
        assert_eq!(f.path.as_os_str(), "src/RFC-0173_Versioning.md");
    }
}
