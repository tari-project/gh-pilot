use crate::newtype;

mod issue_id;
mod new_label;

pub use issue_id::IssueId;
pub use new_label::NewLabel;

newtype!(GithubHandle, String, str);
