use serde::{ Deserialize, Serialize };

use crate::models::{
    common::{License, Permissions, Url, Visibility},
    user::SimpleUser,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryReference {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Repository {
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
    pub description: String,
    pub downloads_url: Url,
    pub events_url: Url,
    pub fork: bool,
    pub forks_url: Url,
    pub full_name: String,
    pub git_commits_url: Url,
    pub git_refs_url: Url,
    pub git_tags_url: Url,
    pub hooks_url: Url,
    pub html_url: Url,
    pub id: u64,
    pub node_id: String,
    pub issue_comment_url: Url,
    pub issue_events_url: Url,
    pub issues_url: Url,
    pub keys_url: Url,
    pub labels_url: Url,
    pub languages_url: Url,
    pub merges_url: Url,
    pub milestones_url: Url,
    pub name: String,
    pub notifications_url: Url,
    pub owner: SimpleUser,
    pub private: bool,
    pub pulls_url: Url,
    pub releases_url: Url,
    pub stargazers_url: Url,
    pub statuses_url: Url,
    pub subscribers_url: Url,
    pub subscription_url: Url,
    pub tags_url: Url,
    pub teams_url: Url,
    pub trees_url: Url,
    pub url: Url,
    pub clone_url: Url,
    pub default_branch: String,
    pub forks: u64,
    pub forks_count: u64,
    pub git_url: Url,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub homepage: String,
    pub language: String,
    pub master_branch: Option<String>,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: Visibility,
    pub mirror_url: Option<Url>,
    pub open_issues: u64,
    pub open_issues_count: u64,
    pub permissions: Option<Permissions>,
    pub temp_clone_token: Option<String>,
    pub allow_merge_commit: Option<bool>,
    pub allow_squash_merge: Option<bool>,
    pub allow_rebase_merge: Option<bool>,
    pub license: License,
    pub pushed_at: String,
    pub size: u64,
    pub ssh_url: Url,
    pub stargazers_count: u64,
    pub svn_url: Url,
    pub topics: Vec<String>,
    pub watchers: u64,
    pub watchers_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub allow_forking: bool,
    pub is_template: bool,
}

#[cfg(test)]
mod test {
    use crate::models::{repository::Repository, static_data::repositories::TARI_REPO};

    #[test]
    fn tari_repo_deserializes() {
        let tari: Repository = serde_json::from_str(TARI_REPO).unwrap();
        assert_eq!(tari.id, 136459099);
        assert_eq!(tari.name, "tari");
        assert_eq!(tari.owner.node_id, "MDEyOk9yZ2FuaXphdGlvbjM3NTYwNTM5");
        assert_eq!(tari.license.spdx_id, "BSD-3-Clause");
    }
}
