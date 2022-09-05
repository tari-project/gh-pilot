use async_trait::async_trait;
use crate::pub_sub::GithubEventMessage;

#[async_trait]
pub trait Action: Send + Sync {
    async fn run(&self, msg: GithubEventMessage);
}