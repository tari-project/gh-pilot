use crate::models::common::Link;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Links {
    pub comments: Link,
    pub commits: Link,
    pub statuses: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comments: Link,
    pub review_comment: Link,
    #[serde(rename(deserialize = "self"))]
    pub ref_self: Link,
}
