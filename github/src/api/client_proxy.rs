use crate::api::{AuthToken, GithubApiError};
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

pub const USER_AGENT: &str = "GithubPilot_v1.0";
pub const BASE_URL: &str = "https://api.github.com";

#[derive(Clone)]
pub struct ClientProxy {
    client: Client,
    auth: Option<(String, AuthToken)>,
}

impl Default for ClientProxy {
    fn default() -> Self {
        let client = Client::builder().user_agent(USER_AGENT).build().unwrap(); // FIXME! Deal with this
        ClientProxy { client, auth: None }
    }
}

impl ClientProxy {
    pub fn new(username: &str, auth: AuthToken) -> Self {
        let mut this = ClientProxy::default();
        this.with_auth(username, auth);
        this
    }

    pub fn with_auth(&mut self, user: &str, auth: AuthToken) {
        self.auth = Some((user.into(), auth));
    }

    pub fn apply_auth(&self, request: RequestBuilder) -> RequestBuilder {
        match &self.auth {
            Some((user, auth)) => request.basic_auth(user, Some(auth)),
            None => request,
        }
    }

    pub fn request<S: AsRef<str>>(&self, method: Method, path: S, auth: bool) -> RequestBuilder {
        let url = [BASE_URL, path.as_ref()].join("");
        let request = self.client.request(method, url);
        
        if auth {
            self.apply_auth(request)
        } else {
            request
        }
    }

    pub fn get<S: AsRef<str>>(&self, url: S, auth: bool) -> RequestBuilder {
        self.request(Method::GET, url, auth)
    }

    pub async fn send<T: DeserializeOwned>(
        &self,
        request: RequestBuilder,
    ) -> Result<T, GithubApiError> {
        let response = request
            .send()
            .await
            .map_err(|e| GithubApiError::HttpClientError(e.to_string()))?;
        match response.status() {
            StatusCode::OK => response
                .json()
                .await
                .map_err(|e| GithubApiError::DeserializationError(e.to_string())),
            StatusCode::NOT_FOUND => Err(GithubApiError::NotFound(
                response.text().await.unwrap_or_else(|_| "Not found".into()),
            )),
            code => Err(GithubApiError::HttpResponse(code)),
        }
    }
}
