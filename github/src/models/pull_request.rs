use crate::models::common::Url;
use crate::models::git::GitReference;
use crate::models::labels::Label;
use crate::models::links::Links;
use crate::models::team::SimpleTeam;
use crate::models::user::SimpleUser;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum State {
    Open,
    Closed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
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

#[derive(Debug, Deserialize)]
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
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub merged_at: String,
    pub merge_commit_sha: String,
    pub assignee: Option<SimpleUser>,
    pub assignees: Vec<SimpleUser>,
    pub requested_reviewers: Vec<SimpleUser>,
    pub requested_teams: Option<Vec<SimpleTeam>>,
    pub head: GitReference,
    pub base: GitReference,
    #[serde(rename(deserialize = "_links"))]
    pub links: Links,
    pub author_association: AuthorAssociation,
    pub auto_merge: Option<bool>,
    pub draft: bool,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: Option<SimpleUser>,
    pub comments: usize,
    pub review_comments: usize,
    pub maintainer_can_modify: bool,
    pub commits: usize,
    pub additions: usize,
    pub deletions: usize,
    pub changed_files: usize,
}

#[cfg(test)]
mod test {
    use crate::models::pull_request::PullRequest;
    use crate::models::repository::Repository;
    use crate::models::static_data::pull_requests::TARI_PR_1K;
    use crate::models::user::SimpleUser;


    #[test]
    fn tari_pr_1000() {
        let pr: PullRequest = serde_json::from_str(TARI_PR_1K).unwrap();
        assert_eq!(pr.comments, 2);
        assert_eq!(pr.id, 338616778);
        assert_eq!(pr.merged, true);
        assert!(matches!(pr.merged_by, Some(SimpleUser{login, ..}) if login == "CjS77"));
        assert_eq!(
            pr.links.commits.href,
            "https://api.github.com/repos/tari-project/tari/pulls/1000/commits"
        );
        assert!(
            matches!(pr.base.repo, Some(Repository { description, ..}) if description == "The Tari protocol")
        );
    }
}
