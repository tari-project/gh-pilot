use crate::models::integration::Integration;
use crate::models::milestone::Milestone;
use crate::models::{
    AuthorAssociation, IssuePullRequest, Label, Reactions, Repository, SimpleUser, State, Url,
};
use serde::Deserialize;

/// Issue : Issues are a great way to keep track of tasks, enhancements, and bugs for your projects.
#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub node_id: String,
    /// URL for the issue
    pub url: Url,
    pub repository_url: Url,
    pub labels_url: Url,
    pub comments_url: Url,
    pub events_url: Url,
    pub html_url: Url,
    /// Number uniquely identifying the issue within its repository
    pub number: u64,
    /// State of the issue; either 'open' or 'closed'
    pub state: State,
    /// The reason for the current state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// Title of the issue
    pub title: String,
    /// Contents of the issue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    pub user: Option<Box<SimpleUser>>,
    /// Labels to associate with this issue; pass one or more label names to replace the set of labels on this issue; send an empty array to clear all labels from the issue; note that the labels are silently dropped for users without push access to the repository
    pub labels: Vec<Label>,
    pub assignee: Option<Box<SimpleUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignees: Option<Vec<SimpleUser>>,
    pub milestone: Option<Box<Milestone>>,
    pub locked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_lock_reason: Option<String>,
    pub comments: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Box<IssuePullRequest>>,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_by: Option<Box<SimpleUser>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeline_url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<Repository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<Box<Integration>>,
    #[serde(rename = "author_association")]
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Box<Reactions>>,
}

impl Issue {
    pub fn user(&self) -> Option<&SimpleUser> {
        self.user.as_deref()
    }
}

#[cfg(test)]
mod test {
    use crate::models::issues::Issue;
    use crate::models::static_data::issues::ISSUE;
    use crate::models::{SimpleUser, State};

    #[test]
    fn deserialize_issue() {
        let issue: Issue = serde_json::from_str(ISSUE).unwrap();
        assert_eq!(issue.title, "Add elliptic curve marker trait?");
        assert_eq!(issue.number, 72);
        assert!(matches!(issue.state, State::Open));
        assert!(matches!(issue.user(), Some(SimpleUser { login, ..}) if login == "CjS77"));
    }
}
