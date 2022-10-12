use log::{debug, trace};

use crate::{
    models::{CheckSuite, CheckSuiteEvent},
    wrappers::IssueId,
};

impl CheckSuite {
    pub fn summary(&self) -> String {
        let prs = self
            .pull_requests
            .iter()
            .map(|pr| format!("PR#{}", pr.id))
            .collect::<Vec<String>>();
        let status = self.status.map(|s| s.to_string()).unwrap_or_else(|| "N/A".to_string());
        let conclusion = self
            .conclusion
            .map(|c| c.to_string())
            .unwrap_or_else(|| "N/A".to_string());
        format!(
            "Check suite {} for PRs {} {status}: {conclusion}",
            self.id,
            prs.join(",")
        )
    }
}

impl CheckSuiteEvent {
    /// Returns the set of associated PRs as a vector of IssueId
    pub fn related_pull_requests(&self) -> Vec<IssueId> {
        let owner = match self.info.organization {
            Some(ref org) => org.login.as_str(),
            None => {
                trace!(
                    "No organization was attached in the check suite event, so can't pull owner. If this is an issue, \
                     we _could_ try extract the info from the pr url field, but that's obviously a bit brittle."
                );
                return vec![];
            },
        };
        let repo = self.info.repository.name.as_str();
        self.check_suite
            .pull_requests
            .iter()
            .map(|pr| IssueId::new(owner, repo, pr.number))
            .collect()
    }

    pub fn first_related_pr(&self) -> Option<IssueId> {
        let prs = self.related_pull_requests();
        match prs.len() {
            0 => None,
            1 => prs.first().cloned(),
            _ => {
                debug!(
                    "CheckSuiteEvent related to multiple PRs: {:?}. Returning the first one",
                    prs.len()
                );
                prs.first().cloned()
            },
        }
    }
}
