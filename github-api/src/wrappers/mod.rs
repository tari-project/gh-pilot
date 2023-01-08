use std::fmt::{Display, Formatter};

use crate::newtype;

mod issue_id;
mod new_label;
mod repo_id;

pub use issue_id::IssueId;
pub use new_label::NewLabel;
pub use repo_id::{RepoId, RepoIdParseError};

newtype!(GithubHandle, String, str);

impl Display for GithubHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}
