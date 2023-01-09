use async_trait::async_trait;
use log::trace;

use crate::{
    error::GithubProviderError,
    graphql::org_activity::{org_activity_ql::pageInfoFields, OrgActivity, OrgActivitySearch},
};

#[async_trait]
pub trait OrganizationProvider: Sync {
    async fn fetch_activity(
        &self,
        owner: &str,
        from: &str,
        to: &str,
        n: usize,
        next_from: Option<pageInfoFields>,
    ) -> Result<OrgActivitySearch, GithubProviderError>;

    async fn fetch_activities(&self, owner: &str, from: &str, to: &str) -> Result<OrgActivity, GithubProviderError> {
        let mut more = true;
        let mut result = OrgActivity::default();
        const BATCH_SIZE: usize = 100;
        let mut page_info = None;
        while more {
            let search = self.fetch_activity(owner, from, to, BATCH_SIZE, page_info).await?;
            trace!("Search info: {:?}", search.page_info);
            more = search.page_info.has_next_page;
            page_info = Some(search.page_info);
            result.merge(search.search_result);
        }
        Ok(result)
    }
}
