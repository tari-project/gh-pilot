use crate::newtype;

mod issue_id;
pub use issue_id::IssueId;

newtype!(GithubHandle, String, str);
