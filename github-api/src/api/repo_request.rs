use reqwest::StatusCode;

use crate::{
    api::{error::ErrorItem, ClientProxy, GithubApiError},
    models::{Contributor, Label, Repository, UserType},
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
        let res = proxy
            .delete(url)
            .send()
            .await
            .map_err(|e| GithubApiError::ReqwestError(e.to_string()))?;
        Ok(res.status().is_success())
    }

    pub async fn assign_labels(&self, proxy: &ClientProxy, labels: &[NewLabel]) -> Result<(), GithubApiError> {
        let url = format!("/repos/{}/{}/labels", self.owner, self.repo);
        let mut errors = vec![];
        for label in labels {
            let req = proxy.post(url.as_str()).json(label);
            let res = req
                .send()
                .await
                .map_err(|e| GithubApiError::ReqwestError(e.to_string()))?;
            let status = res.status();
            // Any success code, or 422 (already exists) is ok, just carry on.
            if status.is_success() || status == StatusCode::UNPROCESSABLE_ENTITY {
                continue;
            }
            let err = res
                .text()
                .await
                .unwrap_or_else(|_| "No error text provided by Github".to_string());
            errors.push(ErrorItem::new(
                label.name.as_str(),
                GithubApiError::HttpClientError(err),
            ));
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(GithubApiError::MultipleErrors(errors))
        }
    }

    pub async fn edit_label(&self, proxy: &ClientProxy, label: &str, new: &NewLabel) -> Result<bool, GithubApiError> {
        let url = format!("/repos/{}/{}/labels/{label}", self.owner, self.repo);
        let req = proxy.patch(url).json(new);
        proxy.send(req).await
    }

    pub async fn fetch_contributors(&self, proxy: &ClientProxy) -> Result<Vec<Contributor>, GithubApiError> {
        let url = format!("/repos/{}/{}/contributors", self.owner, self.repo);
        let req = proxy.get(&url, false);
        let contributors: Vec<Contributor> = proxy.send(req).await?;
        let contributors = contributors
            .into_iter()
            .filter(|c| !c.login.is_empty() && matches!(c.user_type, Some(UserType::User)))
            .collect::<Vec<Contributor>>();
        Ok(contributors)
    }
}
