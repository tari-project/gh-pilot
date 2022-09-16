use serde::{Deserialize, Serialize};

use crate::models::{
    common::{License, Permissions, Url, Visibility},
    DateTime,
    SimpleUser,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryReference {
    pub id: u64,
    pub name: String,
    pub url: Url,
}

/// Repository struct as returned by the Github API. The events API returns the repo data in a different format. See
/// [`Repository2`]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repository {
    /// Unique identifier of the repository
    pub id: i32,
    pub node_id: String,
    /// The name of the repository.
    pub name: String,
    pub full_name: String,
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<SimpleUser>,
    pub forks: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub owner: SimpleUser,
    /// Whether the repository is private or public.
    pub private: bool,
    pub html_url: Url,
    pub description: Option<String>,
    pub fork: bool,
    pub url: Url,
    pub archive_url: Url,
    pub assignees_url: Url,
    pub blobs_url: Url,
    pub branches_url: Url,
    pub collaborators_url: Url,
    pub comments_url: Url,
    pub commits_url: Url,
    pub compare_url: Url,
    pub contents_url: Url,
    pub contributors_url: Url,
    pub deployments_url: Url,
    pub downloads_url: Url,
    pub events_url: Url,
    pub forks_url: Url,
    pub git_commits_url: Url,
    pub git_refs_url: Url,
    pub git_tags_url: Url,
    pub git_url: Url,
    pub issue_comment_url: Url,
    pub issue_events_url: Url,
    pub issues_url: Url,
    pub keys_url: Url,
    pub labels_url: Url,
    pub languages_url: Url,
    pub merges_url: Url,
    pub milestones_url: Url,
    pub notifications_url: Url,
    pub pulls_url: Url,
    pub releases_url: Url,
    pub ssh_url: Url,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub tags_url: Url,
    pub teams_url: Url,
    pub trees_url: Url,
    pub clone_url: Url,
    pub mirror_url: Option<Url>,
    pub hooks_url: Url,
    pub svn_url: Url,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: i32,
    pub stargazers_count: i32,
    pub watchers_count: i32,
    pub size: i32,
    /// The default branch of the repository.
    pub default_branch: String,
    pub open_issues_count: i32,
    /// Whether this repository acts as a template that can be used to generate new repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_template: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Whether issues are enabled.
    pub has_issues: bool,
    /// Whether projects are enabled.
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    pub has_wiki: bool,
    pub has_pages: bool,
    /// Whether downloads are enabled.
    pub has_downloads: bool,
    /// Whether the repository is archived.
    pub archived: bool,
    /// Returns whether or not this repository disabled.
    pub disabled: bool,
    /// The repository visibility: public, private, or internal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,
    pub pushed_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    // pub template_repository: Option<_>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    /// Whether to allow squash merges for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    /// Whether to allow Auto-merge to be used on pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_auto_merge: Option<bool>,
    /// Whether to delete head branches when pull requests are merged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_branch_on_merge: Option<bool>,
    /// Whether or not a pull request head branch that is behind its base branch can always be updated even if it is
    /// not required to be up to date before merging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_update_branch: Option<bool>,
    /// Whether a squash merge commit can use the pull request title as default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_squash_pr_title_as_default: Option<bool>,
    /// Whether to allow merge commits for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow forking this repo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_forking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_count: Option<i32>,
    pub open_issues: i32,
    pub watchers: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<DateTime>,
}

/// Repository data as returned by the Events API. For the Github API struct see [`Repository`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository2 {
    /// Whether to allow merge commits for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_merge_commit: Option<bool>,
    /// Whether to allow rebase merges for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_rebase_merge: Option<bool>,
    /// Whether to allow squash merges for pull requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_squash_merge: Option<bool>,
    pub archive_url: Url,
    /// Whether the repository is archived.
    pub archived: bool,
    pub assignees_url: Url,
    pub blobs_url: Url,
    pub branches_url: Url,
    pub clone_url: Url,
    pub collaborators_url: Url,
    pub comments_url: Url,
    pub commits_url: Url,
    pub compare_url: Url,
    pub contents_url: Url,
    pub contributors_url: Url,
    pub created_at: DateTime,
    /// The default branch of the repository.
    pub default_branch: String,
    /// Whether to delete head branches when pull requests are merged
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: Url,
    pub description: Option<String>,
    /// Returns whether or not this repository is disabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: Url,
    pub events_url: Url,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: Url,
    pub full_name: String,
    pub git_commits_url: Url,
    pub git_refs_url: Url,
    pub git_tags_url: Url,
    pub git_url: Url,
    /// Whether downloads are enabled.
    pub has_downloads: bool,
    /// Whether issues are enabled.
    pub has_issues: bool,
    pub has_pages: bool,
    /// Whether projects are enabled.
    pub has_projects: bool,
    /// Whether the wiki is enabled.
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: Url,
    pub html_url: Url,
    /// Unique identifier of the repository
    pub id: i64,
    pub issue_comment_url: Url,
    pub issue_events_url: Url,
    pub issues_url: Url,
    pub keys_url: Url,
    pub labels_url: Url,
    pub language: Option<String>,
    pub languages_url: Url,
    pub license: Option<License>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: Url,
    pub milestones_url: Url,
    pub mirror_url: Option<Url>,
    /// The name of the repository.
    pub name: String,
    pub node_id: String,
    pub notifications_url: Url,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: SimpleUser,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    /// Whether the repository is private or public.
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: Url,
    pub pushed_at: DateTime,
    pub releases_url: Url,
    pub size: i64,
    pub ssh_url: Url,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub svn_url: Url,
    pub tags_url: Url,
    pub teams_url: Url,
    pub trees_url: Url,
    pub updated_at: DateTime,
    pub url: Url,
    pub watchers: i64,
    pub watchers_count: i64,
}

#[cfg(test)]
mod test {
    use crate::models::repository::Repository;

    #[test]
    fn tari_repo_deserializes() {
        let data = include_str!("../test_data/tari_repo.json");
        let tari: Repository = serde_json::from_str(data).unwrap();
        assert_eq!(tari.id, 136459099);
        assert_eq!(tari.name, "tari");
        assert_eq!(tari.owner.node_id, "MDEyOk9yZ2FuaXphdGlvbjM3NTYwNTM5");
        assert_eq!(tari.license.unwrap().spdx_id, "BSD-3-Clause");
    }
}
