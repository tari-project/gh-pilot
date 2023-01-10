use async_trait::async_trait;
use log::*;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;

use crate::api::{AuthToken, GithubApiError, GithubQuery, GithubQueryExec, Page};

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
        let url = [BASE_URL, path.as_ref()].concat();
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

    pub fn post<S: AsRef<str>>(&self, url: S) -> RequestBuilder {
        self.request(Method::POST, url, true)
    }

    pub fn delete<S: AsRef<str>>(&self, url: S) -> RequestBuilder {
        self.request(Method::DELETE, url, true)
    }

    pub fn put<S: AsRef<str>>(&self, url: S) -> RequestBuilder {
        self.request(Method::PUT, url, true)
    }

    pub fn patch<S: AsRef<str>>(&self, url: S) -> RequestBuilder {
        self.request(Method::PATCH, url, true)
    }

    pub async fn send<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T, GithubApiError> {
        let response = request
            .send()
            .await
            .map_err(|e| GithubApiError::HttpClientError(e.to_string()))?;
        trace!("Response status: {}", response.status());
        trace!("Response headers: {:?}", response.headers());
        match response.status() {
            StatusCode::OK |
            StatusCode::CREATED |
            StatusCode::ACCEPTED |
            StatusCode::PARTIAL_CONTENT |
            StatusCode::RESET_CONTENT |
            StatusCode::MULTI_STATUS |
            StatusCode::ALREADY_REPORTED => response
                .json()
                .await
                .map_err(|e| GithubApiError::DeserializationError(e.to_string())),
            StatusCode::NOT_FOUND => Err(GithubApiError::NotFound(
                response.text().await.unwrap_or_else(|_| "Not found".into()),
            )),
            code => Err(GithubApiError::HttpResponse(code)),
        }
    }

    /// Fetch paginated results, stopping when a given filter fails
    ///
    /// The method sends HTTP requests to an endpoint and returns a list of records of type T that match the
    /// specified criteria. The method will keep sending requests until there are no more records that match the
    /// criteria. The requests are paginated, with each page having a length of `page_len`. The closure `filter` is used
    /// to filter the records received in each page before adding them to the result list. If an error occurs during
    /// the process, and if there are any records in the result list at the time of the error, the method will return
    ///  the set of results it has already collected.
    ///
    /// # Parameters:
    ///
    ///  * req: a RequestBuilder object. use [ClientProxy::get], [ClientProxy::post] etc to create this
    ///  * filter: a closure with a single parameter &T that returns a bool. Every element returned from each query will
    /// be evaluated against this function. If `filter` returns true, the record will be included in the results,
    /// otherwise it will not. As soon as `filter` returns false, the requests will stop (but the current page will
    /// be fully tested against the filter)
    ///  * page_len: the length of a page for each request
    pub async fn fetch_pages<F, T>(
        &self,
        req: RequestBuilder,
        filter: F,
        page_len: usize,
    ) -> Result<Vec<T>, GithubApiError>
    where
        F: Fn(&T) -> bool,
        T: DeserializeOwned,
    {
        let mut page = 1;
        let mut done = false;
        let mut result = vec![];
        while !done {
            // the events endpoint does not respect the "since" parameter
            let q = Page::nth(page, page_len).to_query();
            let request = req
                .try_clone()
                .ok_or_else(|| GithubApiError::HttpClientError("Could not clone request".into()))?
                .query(&q);
            let response = match request.send().await {
                Ok(res) => res,
                Err(e) => {
                    let url = e.url().map(|u| u.as_str()).unwrap_or_default();
                    warn!("Error fetching paged result. {e}. The request was {url}");
                    // If we already have results, return what we have
                    return if result.is_empty() {
                        Err(GithubApiError::HttpClientError(e.to_string()))
                    } else {
                        Ok(result)
                    };
                },
            };
            match response.json::<Vec<T>>().await {
                Ok(records) => {
                    done = records.len() < page_len;
                    let new_records = records.into_iter().filter(|rec| {
                        let include_rec = filter(rec);
                        if !include_rec {
                            done = true;
                        }
                        include_rec
                    });
                    result.extend(new_records);
                },
                Err(e) => {
                    let url = e.url().map(|u| u.as_str()).unwrap_or_default();
                    warn!("Error converting results. {e}. The request was {url}",);
                    done = true;
                    // If we have results, we can return an error
                    if result.is_empty() {
                        return Err(GithubApiError::HttpClientError(e.to_string()));
                    }
                },
            }
            page += 1;
        }
        Ok(result)
    }
}

#[async_trait]
impl GithubQueryExec for ClientProxy {
    type Error = GithubApiError;

    async fn exec<T: DeserializeOwned + Send>(&self, request: impl GithubQuery) -> Result<T, Self::Error> {
        let req = self.request(request.method(), request.path(), request.use_auth());
        self.send::<T>(req).await
    }
}
