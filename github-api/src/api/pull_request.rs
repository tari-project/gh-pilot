use graphql_client::{GraphQLQuery, Response};

use crate::{
    api::{ClientProxy, GithubApiError, IssueRequest},
    graphql::{
        pr_comments::{pull_request_comments_ql, PullRequestCommentsQL},
        PullRequestComments,
    },
    models::{Label, PullRequest},
    models_plus::{MergeParameters, MergeResult},
    wrappers::IssueId,
};

pub struct PullRequestRequest {
    owner: String,
    repo: String,
    pull: u64,
    url: String,
}

impl From<&IssueId> for PullRequestRequest {
    fn from(id: &IssueId) -> Self {
        Self::new(&id.owner, &id.repo, id.number)
    }
}

impl PullRequestRequest {
    pub fn new<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> Self {
        Self {
            owner: owner.as_ref().into(),
            repo: repo.as_ref().into(),
            pull,
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
        self.pull
    }

    fn url<S: AsRef<str>>(owner: S, repo: S, pull: u64) -> String {
        format!("/repos/{}/{}/pulls/{}", owner.as_ref(), repo.as_ref(), pull)
    }

    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<PullRequest, GithubApiError> {
        let req = proxy.get(self.url.as_str(), false);
        proxy.send(req).await
    }

    pub async fn add_labels(&self, labels: &[&str], proxy: &ClientProxy) -> Result<Vec<Label>, GithubApiError> {
        // prs are also issues
        let issue = IssueRequest::new(&self.owner, &self.repo, self.pull);
        issue.add_labels(labels, proxy).await
    }

    pub async fn remove_label(&self, label: &str, proxy: &ClientProxy) -> Result<Vec<Label>, GithubApiError> {
        // prs are also issues
        let issue = IssueRequest::new(&self.owner, &self.repo, self.pull);
        issue.remove_label(label, proxy).await
    }

    pub async fn fetch_comments(&self, proxy: &ClientProxy) -> Result<PullRequestComments, GithubApiError> {
        let vars = pull_request_comments_ql::Variables {
            owner: self.owner.clone(),
            repo: self.repo.clone(),
            pr_number: self.pull as i64,
        };
        let body = PullRequestCommentsQL::build_query(vars);
        let req = proxy.post("/graphql").json(&body);
        let response: Response<pull_request_comments_ql::ResponseData> = proxy.send(req).await?;
        if let Some(data) = response.data {
            Ok(data.into())
        } else {
            match response.errors {
                None => Err(GithubApiError::DeserializationError(
                    "No data came back in the response".into(),
                )),
                Some(errs) => Err(GithubApiError::GraphQLError(
                    errs.into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join("; "),
                )),
            }
        }
    }

    pub async fn merge(&self, proxy: &ClientProxy, params: MergeParameters) -> Result<MergeResult, GithubApiError> {
        let url = format!("{}/merge", self.url);
        let req = proxy.put(url.as_str()).json(&params);
        match proxy.send::<MergeResult>(req).await {
            Ok(r) => Ok(r),
            Err(GithubApiError::HttpResponse(code)) if code == 405 => {
                let msg = format!("The PR {}/{}#{} could not be merged.", self.owner, self.repo, self.pull);
                Err(GithubApiError::MergeError(msg))
            },
            Err(e) => Err(e),
        }
    }
}
