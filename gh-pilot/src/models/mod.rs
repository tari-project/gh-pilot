use ghp_api::newtype;

mod issue_id;

newtype!(GithubHandle, String, str);

pub use issue_id::IssueId;
