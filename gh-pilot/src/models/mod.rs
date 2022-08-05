#[macro_use]
mod newtype;
// github models
mod user;

//pub use gh_handle::GithubHandle;
pub use user::{AuthenticatedUser, UserDetails};

// pub struct GithubHandle(String);
newtype!(GithubHandle, String, str);
