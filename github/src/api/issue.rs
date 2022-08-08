use crate::api::{ClientProxy, GithubApiError};
use crate::models::Issue;

pub struct IssueRequest {
    owner: String,
    repo: String,
    number: u64,
    url: String,
}

impl IssueRequest {
    pub fn new<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> Self {
        Self {
            owner: owner.as_ref().into(),
            repo: repo.as_ref().into(),
            number: pull,
            url: Self::url(owner, repo, pull),
        }
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub fn number(&self) -> u64 {
        self.number
    }

    fn url<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> String {
        format!("/repos/{}/{}/issues/{}", owner.as_ref(), repo.as_ref(), pull)
    }
    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<Issue, GithubApiError> {
        let req = proxy.get(self.url.as_str(), false);
        proxy.send(req).await
    }
}
