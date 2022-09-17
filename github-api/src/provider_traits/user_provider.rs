use crate::error::GithubProviderError;
use crate::models::SimpleUser;
use crate::wrappers::GithubHandle;
use async_trait::async_trait;

#[async_trait]
pub trait UserProvider {
    async fn fetch_details(&self, handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubProviderError>;
}