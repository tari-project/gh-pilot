use async_trait::async_trait;

use crate::{error::GithubProviderError, models::SimpleUser, wrappers::GithubHandle};

#[async_trait]
pub trait UserProvider {
    async fn fetch_details(&self, handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubProviderError>;
}
