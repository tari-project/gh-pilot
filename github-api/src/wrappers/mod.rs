use crate::newtype;

mod issue_id;
mod new_label;
mod repo_id;

pub use issue_id::IssueId;
pub use new_label::NewLabel;
pub use repo_id::{RepoId, RepoIdParseError};

newtype!(GithubHandle, String, str);
