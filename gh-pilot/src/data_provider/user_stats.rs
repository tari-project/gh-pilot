//! The generic interface for the data provider api

use crate::error::GithubPilotError;
use crate::models::{GithubHandle, UserDetails};
use async_trait::async_trait;

#[async_trait]
pub trait UserStatsProvider {
    async fn fetch_details(
        &self,
        handle: &GithubHandle,
    ) -> Result<Option<UserDetails>, GithubPilotError>;
    // async fn fetch_contribution_stats(request: UserContributionRequest) -> UserContributionResult;
}
