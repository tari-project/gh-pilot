use crate::{
    api::{ClientProxy, GithubApiError},
    models::{Label, Repository},
    wrappers::NewLabel,
};

pub struct RepoRequest {
    owner: String,
    repo: String,
}

impl RepoRequest {
    pub fn new<S: Into<String>, R: Into<String>>(owner: S, repo: R) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    pub fn repo(&self) -> &str {
        self.repo.as_str()
    }

    pub fn owner(&self) -> &str {
        self.owner.as_str()
    }

    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<Repository, GithubApiError> {
        let url = format!("/repo/{}/{}", self.owner, self.repo);
        let req = proxy.get(&url, false);
        proxy.send(req).await
    }

    pub async fn fetch_labels(
        &self,
        proxy: &ClientProxy,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<Label>, GithubApiError> {
        let url = format!("/repos/{}/{}/labels", self.owner, self.repo);
        let req = proxy.get(&url, false).query(&[("page", page), ("per_page", per_page)]);
        proxy.send(req).await
    }

    pub async fn delete_label(&self, proxy: &ClientProxy, label: &str) -> Result<bool, GithubApiError> {
        let url = format!("/repos/{}/{}/labels/{label}", self.owner, self.repo);
        let req = proxy.delete(url);
        proxy.send(req).await
    }

    pub async fn assign_labels(&self, proxy: &ClientProxy, labels: &[NewLabel]) -> Result<(), GithubApiError> {
        let url = format!("/repos/{}/{}/labels", self.owner, self.repo);
        let req = proxy.post(url).json(labels);
        proxy.send(req).await
    }

    pub async fn edit_label(&self, proxy: &ClientProxy, label: &str, new: &NewLabel) -> Result<bool, GithubApiError> {
        let url = format!("/repos/{}/{}/labels/{label}", self.owner, self.repo);
        let req = proxy.patch(url).json(new);
        proxy.send(req).await
    }
}
