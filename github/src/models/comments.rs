use serde::{ Deserialize, Serialize };

use crate::models::{common::Url, pull_request::AuthorAssociation, user::SimpleUser};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reactions {
    pub url: Url,
    pub total_count: u64,
    #[serde(rename = "+1")]
    pub plus1: u64,
    #[serde(rename = "-1")]
    pub minus1: u64,
    pub laugh: u64,
    pub hooray: u64,
    pub confused: u64,
    pub heart: u64,
    pub rocket: u64,
    pub eyes: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommitComment {
    pub html_url: Url,
    pub url: Url,
    pub id: u64,
    pub node_id: String,
    pub body: String,
    pub path: Option<String>,
    pub position: Option<u64>,
    pub line: Option<u64>,
    pub commit_id: String,
    pub user: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: AuthorAssociation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Reactions>,
}

#[cfg(test)]
mod test {
    use crate::models::{comments::IssueComment, static_data::comments::COMMENT};
    #[test]
    fn comment1() {
        let comment: IssueComment = serde_json::from_str(COMMENT).unwrap();
        assert_eq!(comment.node_id, "MDI0OlB1bGxSZXF1ZXN0UmV2aWV3Q29tbWVudDM0NDExNjkzNA==");
        assert_eq!(comment.body.unwrap(), "Current RFC says Gen is height 1, not 0");
        assert_eq!(
            comment.pull_request_url.unwrap(),
            "https://api.github.com/repos/tari-project/tari/pulls/1000"
        );
        assert_eq!(comment.reactions.plus1, 12);
    }
}
