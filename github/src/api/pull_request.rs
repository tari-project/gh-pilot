use crate::api::{ClientProxy, GithubApiError};
use crate::models::{PullRequest};

pub struct PullRequestRequest {
    owner: String,
    repo: String,
    pull: u64,
    url: String,
}

impl PullRequestRequest {
    pub fn new<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> Self {
        Self {
            owner: owner.as_ref().into(),
            repo: repo.as_ref().into(),
            pull,
            url: Self::url(owner, repo, pull)
        }
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn number(&self) -> u64 {
        self.pull
    }

    fn url<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> String {
        format!("/repos/{}/{}/pulls/{}", owner.as_ref(), repo.as_ref(), pull)
    }
    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<PullRequest, GithubApiError> {
        let req = proxy.get(self.url.as_str(), false);
        proxy.send(req).await
    }
}