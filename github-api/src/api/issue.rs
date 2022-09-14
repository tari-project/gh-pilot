use serde::Serialize;

use crate::{
    api::{ClientProxy, GithubApiError},
    models::{Issue, Label},
};

pub struct IssueRequest {
    owner: String,
    repo: String,
    number: u64,
}

#[derive(Serialize)]
struct LabelBody {
    labels: Vec<String>,
}

impl From<&[&str]> for LabelBody {
    fn from(src: &[&str]) -> Self {
        let labels = src.iter().map(|&s| s.to_string()).collect();
        Self { labels }
    }
}

impl IssueRequest {
    pub fn new<S: Into<String>>(owner: S, repo: S, number: u64) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            number,
        }
    }

    pub fn fetch_path(&self) -> String {
        format!("/repos/{}/{}/issues/{}", self.owner, self.repo, self.number)
    }

    fn add_label_path(&self) -> String {
        format!("/repos/{}/{}/issues/{}/labels", self.owner, self.repo, self.number)
    }

    fn remove_label_path(&self, label: &str) -> String {
        format!(
            "/repos/{}/{}/issues/{}/labels/{}",
            self.owner, self.repo, self.number, label
        )
    }

    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<Issue, GithubApiError> {
        let req = proxy.get(self.fetch_path().as_str(), false);
        proxy.send(req).await
    }

    pub async fn add_labels(&self, labels: &[&str], proxy: &ClientProxy) -> Result<Vec<Label>, GithubApiError> {
        let body = LabelBody::from(labels);
        let body = serde_json::to_string(&body).map_err(|e| GithubApiError::SerializationError(e.to_string()))?;
        let req = proxy.post(self.add_label_path().as_str()).body(body);
        proxy.send(req).await
    }

    pub async fn remove_label(&self, label: &str, proxy: &ClientProxy) -> Result<Vec<Label>, GithubApiError> {
        let req = proxy.delete(self.remove_label_path(label).as_str());
        proxy.send(req).await
    }
}
