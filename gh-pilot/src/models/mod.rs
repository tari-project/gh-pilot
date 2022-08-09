#[macro_use]
mod newtype;
// github models
mod user;
mod issues;

newtype!(GithubHandle, String, str);

pub use issues::IssueId;