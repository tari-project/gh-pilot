use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::models::{
    common::Url,
    git::GitReference,
    labels::Label,
    links::Links,
    team::SimpleTeam,
    user::SimpleUser,
    DateTime,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum State {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Closed => f.write_str("Closed"),
            State::Open => f.write_str("Open"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE", serialize = "PascalCase"))]
pub enum AuthorAssociation {
    Collaborator,
    Contributor,
    FirstTimer,
    FirstTimeContributor,
    Mannequin,
    Member,
    None,
    Owner,
}

impl ToString for AuthorAssociation {
    fn to_string(&self) -> String {
        match *self {
            Self::Collaborator => "COLLABORATOR".to_string(),
            Self::Contributor => "CONTRIBUTOR".to_string(),
            Self::FirstTimer => "FIRST_TIMER".to_string(),
            Self::FirstTimeContributor => "FIRST_TIME_CONTRIBUTOR".to_string(),
            Self::Mannequin => "MANNEQUIN".to_string(),
            Self::Member => "MEMBER".to_string(),
            Self::None => "NONE".to_string(),
            Self::Owner => "OWNER".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PullRequest {
    pub url: Url,
    pub id: u64,
    pub node_id: String,
    pub html_url: Url,
    pub diff_url: Url,
    pub patch_url: Url,
    pub issue_url: Url,
    pub commits_url: Url,
    pub review_comments_url: Url,
    pub review_comment_url: Url,
    pub comments_url: Url,
    pub statuses_url: Url,
    pub number: usize,
    pub state: State,
    pub locked: bool,
    pub title: String,
    pub user: Option<SimpleUser>,
    pub body: String,
    pub labels: Vec<Label>,
    pub milestone: Option<String>,
    pub active_lock_reason: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub closed_at: Option<DateTime>,
    pub merged_at: Option<DateTime>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<SimpleUser>,
    pub assignees: Vec<SimpleUser>,
    pub requested_reviewers: Vec<SimpleUser>,
    pub requested_teams: Option<Vec<SimpleTeam>>,
    pub head: GitReference,
    pub base: GitReference,
    #[serde(rename = "_links")]
    pub links: Links,
    pub author_association: AuthorAssociation,
    pub auto_merge: Option<bool>,
    pub draft: bool,
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<SimpleUser>,
    pub comments: Option<usize>,
    pub review_comments: Option<usize>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<usize>,
    pub additions: Option<usize>,
    pub deletions: Option<usize>,
    pub changed_files: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuePullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::models::{static_data::pull_requests::TARI_PR_1K, Link, PullRequest, Repository, SimpleUser};

    #[test]
    fn tari_pr_1000() {
        let pr: PullRequest = serde_json::from_str(TARI_PR_1K).unwrap();
        assert_eq!(pr.comments, Some(2));
        assert_eq!(pr.id, 338616778);
        assert_eq!(pr.merged, Some(true));
        assert!(matches!(pr.merged_by, Some(SimpleUser{login, ..}) if login == "CjS77"));
        assert!(matches!(pr.links.commits, Some(Link {href, ..})
            if href == "https://api.github.com/repos/tari-project/tari/pulls/1000/commits"));
        assert!(
            matches!(pr.base.repo, Some(Repository { description: Some(d), ..}) if d == "The Tari \
        protocol")
        );
    }
}
