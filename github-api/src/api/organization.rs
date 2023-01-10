use graphql_client::{GraphQLQuery, Response};
use log::trace;

use crate::{
    api::{ClientProxy, GithubApiError},
    graphql::org_activity::{org_activity_ql, org_activity_ql::pageInfoFields, OrgActivityQL, OrgActivitySearch},
};

pub struct OrganizationRequest {
    owner: String,
}

impl OrganizationRequest {
    pub fn new<S: Into<String>>(owner: S) -> Self {
        Self { owner: owner.into() }
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    /// Fetch some results for an Organization's activity between the given dates.
    ///
    /// ## Parameters
    /// * `to` and `from` are dates in the format `yyyy-mm-dd` and specify the date range to query.
    /// * `n` is the number of records to return
    /// * For pagination, you can provide a [pageInfoFields] record in `next_from`. This record is returned in
    /// [OrgActivitySearch] and can be passed in directly.
    pub async fn fetch_activity(
        &self,
        proxy: &ClientProxy,
        from: &str,
        to: &str,
        n: usize,
        next_from: Option<pageInfoFields>,
    ) -> Result<OrgActivitySearch, GithubApiError> {
        let query = format!("org:{} created:{from}..{to}", self.owner);
        let after = next_from
            .filter(|p| p.has_next_page && p.end_cursor.is_some())
            .map(|p| p.end_cursor.unwrap());
        trace!(
            "Executing Org Activity request on {} from '{from}' to '{to}' from cursor {after:?}",
            self.owner,
        );
        let vars = org_activity_ql::Variables {
            query,
            first: n as i64,
            after,
        };
        let body = OrgActivityQL::build_query(vars);
        let req = proxy.post("/graphql").json(&body);
        let response: Response<org_activity_ql::ResponseData> = proxy.send(req).await?;
        if let Some(data) = response.data {
            let result = OrgActivitySearch::from(data);
            trace!(
                "Org Activity search successful. {} issues and {} PRs / {} total found",
                result.search_result.issues.len(),
                result.search_result.pull_requests.len(),
                result.total_count
            );
            Ok(result)
        } else {
            match response.errors {
                None => Err(GithubApiError::DeserializationError(
                    "No data came back in the Organization Activity response".into(),
                )),
                Some(errs) => Err(GithubApiError::GraphQLError(
                    errs.into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join("; "),
                )),
            }
        }
    }
}
