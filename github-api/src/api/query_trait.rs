use std::error::Error;

use async_trait::async_trait;
use reqwest::Method;
use serde::de::DeserializeOwned;

pub trait GithubQuery: Send {
    fn path(&self) -> &str;
    fn method(&self) -> Method;
    fn use_auth(&self) -> bool;
}

#[async_trait]
pub trait GithubQueryExec {
    type Error: Error;
    async fn exec<T: DeserializeOwned + Send>(&self, request: impl GithubQuery) -> Result<T, Self::Error>;
}
