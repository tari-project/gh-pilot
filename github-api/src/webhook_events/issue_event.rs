//! Additional implementations for IssueEvent are kept here to avoid messing with the code generation tools in models.rs

use crate::{api::IssueRequest, models::Issue, webhook_events::IssuesEvent};

impl IssuesEvent {
    pub fn owner(&self) -> &str {
        self.info.repository.owner.login.as_str()
    }

    pub fn number(&self) -> u64 {
        self.issue.number
    }

    pub fn repo(&self) -> &str {
        self.info.repository.name.as_str()
    }

    pub fn issue(&self) -> &Issue {
        &self.issue
    }

    pub fn to_request(&self) -> IssueRequest {
        IssueRequest::new(self.owner(), self.repo(), self.number())
    }
}

#[cfg(test)]
mod test {
    use crate::webhook_events::models::IssuesEvent;

    #[test]
    fn issue_metadata() {
        let data = include_str!("../test_data/issue_event.json");
        let issue: IssuesEvent = serde_json::from_str(data).unwrap();
        assert_eq!(issue.owner(), "tari-project");
        assert_eq!(issue.repo(), "tari");
        assert_eq!(issue.number(), 4630);
    }

    #[test]
    fn issue_as_request() {
        let data = include_str!("../test_data/issue_event.json");
        let issue: IssuesEvent = serde_json::from_str(data).unwrap();
        let req = issue.to_request();
        assert_eq!(req.fetch_path(), "/repos/tari-project/tari/issues/4630");
    }
}
