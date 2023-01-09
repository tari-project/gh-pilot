use log::*;

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

    pub fn get_events_path(&self) -> String {
        format!("/users/{}/events", self.username)
    }

    pub async fn fetch(&self, proxy: &ClientProxy) -> Result<Option<SimpleUser>, GithubApiError> {
        let req = proxy.get(self.get_path().as_str(), false);
        match proxy.send::<SimpleUser>(req).await {
            Ok(user) => Ok(Some(user)),
            Err(GithubApiError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_events(
        &self,
        proxy: &ClientProxy,
        since: DateTime,
        auth: bool,
    ) -> Result<Vec<Event>, GithubApiError> {
        let filter = |ev: &Event| {
            if let Some(ts) = &ev.info.created_at {
                *ts >= since
            } else {
                debug!("Event did not have a created_at timestamp. Skipping. {ev:?}");
                false
            }
        };
        let req = proxy.get(self.get_events_path().as_str(), auth);
        proxy.fetch_pages(req, filter, 100).await
    }
}
