use async_trait::async_trait;

use crate::{
    error::GithubProviderError,
    models::{DateTime, Event, SimpleUser},
    wrappers::GithubHandle,
};

#[async_trait]
pub trait UserProvider {
    async fn fetch_details(&self, handle: &GithubHandle) -> Result<Option<SimpleUser>, GithubProviderError>;
    async fn fetch_events(
        &self,
        handle: &GithubHandle,
        since: DateTime,
        auth: bool,
    ) -> Result<Vec<Event>, GithubProviderError>;
}
