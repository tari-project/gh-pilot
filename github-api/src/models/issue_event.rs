use serde::{Deserialize, Serialize};

use crate::{
    api::IssueRequest,
    models::{CommonEventFields, Issue, Label, SimpleUser},
};

//----------------------------------         Issues Event        ------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEvent {
    #[serde(flatten)]
    pub action: IssuesEventAction,
    pub issue: Issue,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "action")]
pub enum IssuesEventAction {
    /// issues assigned event. Activity related to an issue. The type of activity is specified in the action property.
    Assigned {
        /// The optional user who was assigned or unassigned from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<SimpleUser>,
    },
    /// issues closed event
    Closed,
    /// issues deleted event
    Deleted,
    /// issues demilestoned event
    Demilestoned,
    /// issues edited event
    Edited {
        changes: IssuesEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues labeled event
    Labeled {
        /// The optional label that was added or removed from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues locked event
    Locked,
    /// issues milestoned event
    Milestoned,
    /// issues opened event
    Opened,
    /// issues pinned event
    Pinned,
    /// issues reopened event
    Reopened,
    /// issues transferred event
    Transferred,
    /// issues unassigned event
    Unassigned {
        /// The optional user who was assigned or unassigned from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<SimpleUser>,
    },
    /// issues unlabeled event
    Unlabeled {
        /// The optional label that was added or removed from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues unlocked event
    Unlocked,
    /// issues unpinned event
    Unpinned,
}

impl ToString for IssuesEventAction {
    fn to_string(&self) -> String {
        match *self {
            Self::Assigned { .. } => "assigned".to_string(),
            Self::Closed => "closed".to_string(),
            Self::Deleted => "deleted".to_string(),
            Self::Demilestoned => "demilestoned".to_string(),
            Self::Edited { .. } => "edited".to_string(),
            Self::Labeled { .. } => "labeled".to_string(),
            Self::Unlabeled { .. } => "unlabeled".to_string(),
            Self::Locked => "locked".to_string(),
            Self::Milestoned => "milestoned".to_string(),
            Self::Unlocked => "unlocked".to_string(),
            Self::Opened => "opened".to_string(),
            Self::Pinned => "pinned".to_string(),
            Self::Unpinned => "unpinned".to_string(),
            Self::Reopened => "reopened".to_string(),
            Self::Unassigned { .. } => "unassigned".to_string(),
            Self::Transferred => "transferred".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChangesTitle {
    /// The previous version of the title.
    pub from: String,
}

/// The changes to the issue.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssuesEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<IssuesEditedChangesTitle>,
}

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
    use crate::models::IssuesEvent;

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
