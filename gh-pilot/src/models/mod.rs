#[macro_use]
mod newtype;
// github models
mod issues;
mod user;

newtype!(GithubHandle, String, str);

pub use issues::IssueId;
