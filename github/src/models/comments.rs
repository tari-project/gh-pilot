use serde::Deserialize;
use crate::models::common::Url;
use crate::models::pull_request::AuthorAssociation;
use crate::models::user::SimpleUser;

#[derive(Debug, Deserialize)]
pub struct Reactions {
    pub url: Url,
    pub total_count: u64,
    #[serde(rename(deserialize="+1"))]
    pub plus1: u64,
    #[serde(rename(deserialize="-1"))]
    pub minus1: u64,
    pub laugh: u64,
    pub hooray: u64,
    pub confused: u64,
    pub heart: u64,
    pub rocket: u64,
    pub eyes: u64,
}

#[derive(Debug, Deserialize)]
pub struct IssueComment {
    pub id: i64,
    pub node_id: String,
    pub url: Url,
    pub body: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub html_url: Url,
    pub user: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
    pub issue_url: Option<Url>,
    pub author_association: AuthorAssociation,
    pub reactions: Reactions,
    pub pull_request_url: Option<String>,
    pub pull_request_review_id: Option<i64>,
    pub diff_hunk: Option<String>,
    pub path: Option<String>,
    pub position: Option<i64>,
    pub original_position: Option<i64>,
    pub commit_id: Option<String>,
    pub original_commit_id: Option<String>,
    pub start_line: Option<u64>,
    pub original_start_line: Option<u64>,
    pub start_side: Option<u64>,
    pub line: Option<u64>,
    pub original_line: Option<u64>,
    pub side: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::models::comments::IssueComment;

    #[test]
    fn comment1() {
        let comment_json = r#"{
        "url": "https://api.github.com/repos/tari-project/tari/pulls/comments/344116934",
        "pull_request_review_id": 314124780,
        "id": 344116934,
        "node_id": "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDM0NDExNjkzNA==",
        "diff_hunk": "@@ -482,8 +482,9 @@ where T: BlockchainBackend\n     pub fn is_new_best_block(&self, block: &Block) -> Result<bool, ChainStorageError> {\n         let (height, parent_hash) = {\n             let db = self.access_metadata()?;\n+            // If the database is empty, the best block must be the genesis block\n             if db.height_of_longest_chain.is_none() {\n-                return Ok(true);\n+                return Ok(block.header.height == 0);",
        "path": "base_layer/core/src/chain_storage/blockchain_database.rs",
        "position": 7,
        "original_position": 7,
        "commit_id": "f862e3d6ed6c9158ad51d77c14af839b1e45489f",
        "original_commit_id": "f862e3d6ed6c9158ad51d77c14af839b1e45489f",
        "user": {
            "login": "SWvheerden",
            "id": 2579813,
            "node_id": "MDQ6VXNlcjI1Nzk4MTM=",
            "avatar_url": "https://avatars.githubusercontent.com/u/2579813?v=4",
            "gravatar_id": "",
            "url": "https://api.github.com/users/SWvheerden",
            "html_url": "https://github.com/SWvheerden",
            "followers_url": "https://api.github.com/users/SWvheerden/followers",
            "following_url": "https://api.github.com/users/SWvheerden/following{/other_user}",
            "gists_url": "https://api.github.com/users/SWvheerden/gists{/gist_id}",
            "starred_url": "https://api.github.com/users/SWvheerden/starred{/owner}{/repo}",
            "subscriptions_url": "https://api.github.com/users/SWvheerden/subscriptions",
            "organizations_url": "https://api.github.com/users/SWvheerden/orgs",
            "repos_url": "https://api.github.com/users/SWvheerden/repos",
            "events_url": "https://api.github.com/users/SWvheerden/events{/privacy}",
            "received_events_url": "https://api.github.com/users/SWvheerden/received_events",
            "type": "User",
            "site_admin": false
        },
        "body": "Current RFC says Gen is height 1, not 0",
        "created_at": "2019-11-08T10:46:22Z",
        "updated_at": "2019-11-08T10:46:30Z",
        "html_url": "https://github.com/tari-project/tari/pull/1000#discussion_r344116934",
        "pull_request_url": "https://api.github.com/repos/tari-project/tari/pulls/1000",
        "author_association": "COLLABORATOR",
        "_links": {
            "self": {
                "href": "https://api.github.com/repos/tari-project/tari/pulls/comments/344116934"
            },
            "html": {
                "href": "https://github.com/tari-project/tari/pull/1000#discussion_r344116934"
            },
            "pull_request": {
                "href": "https://api.github.com/repos/tari-project/tari/pulls/1000"
            }
        },
        "reactions": {
            "url": "https://api.github.com/repos/tari-project/tari/pulls/comments/344116934/reactions",
            "total_count": 0,
            "+1": 12,
            "-1": 0,
            "laugh": 0,
            "hooray": 0,
            "confused": 0,
            "heart": 0,
            "rocket": 0,
            "eyes": 0
        },
        "start_line": null,
        "original_start_line": null,
        "start_side": null,
        "line": 487,
        "original_line": 487,
        "side": "RIGHT"
    }"#;
        let comment: IssueComment = serde_json::from_str(comment_json).unwrap();
        assert_eq!(comment.node_id, "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDM0NDExNjkzNA==");
        assert_eq!(comment.body.unwrap(), "Current RFC says Gen is height 1, not 0");
        assert_eq!(comment.pull_request_url.unwrap(), "https://api.github.com/repos/tari-project/tari/pulls/1000");
        assert_eq!(comment.reactions.plus1, 12);
    }
}