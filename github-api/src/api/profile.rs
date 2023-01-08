use crate::{
    api::{ClientProxy, GithubApiError},
    models::{DateTime, Event, SimpleUser},
};

pub struct UserRequest {
    username: String,
}

impl UserRequest {
    pub fn new<S: Into<String>>(username: S) -> Self {
        Self {
            username: username.into(),
        }
    }

    pub fn get_path(&self) -> String {
        format!("/users/{}", self.username)
    }

    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<Option<SimpleUser>, GithubApiError> {
        let req = proxy.get(self.get_path().as_str(), false);
        match proxy.send::<SimpleUser>(req).await {
            Ok(user) => Ok(Some(user)),
            Err(GithubApiError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_events(&self, proxy: &ClientProxy, since: DateTime) -> Result<Vec<Event>, GithubApiError> {
        todo!()
    }
}
