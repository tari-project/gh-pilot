use serde::{Deserialize, Serialize};

use crate::models::common::Link;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comments: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_comment: Option<Link>,
    #[serde(rename(deserialize = "self"), skip_serializing_if = "Option::is_none")]
    pub ref_self: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<Link>,
}
