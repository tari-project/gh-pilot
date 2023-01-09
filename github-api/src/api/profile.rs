use log::*;

use crate::{
    api::{ClientProxy, GithubApiError, Page},
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

    pub async fn fetch_events(&self, proxy: &ClientProxy, since: DateTime, auth: bool) -> Vec<Event> {
        let mut page = 1;
        const PAGE_LEN: usize = 100;
        let mut done = false;
        let mut result = vec![];
        while !done {
            // the events endpoint does not respect the "since" parameter
            let q = Page::nth(page, PAGE_LEN).to_query();
            let req = proxy.get(self.get_events_path().as_str(), auth).query(&q);
            match proxy.send::<Vec<Event>>(req).await {
                Ok(events) => {
                    done = events.len() < PAGE_LEN;
                    let new_events = events.into_iter().filter(|ev| {
                        if let Some(ts) = &ev.info.created_at {
                            if *ts < since {
                                done = true;
                            }
                            *ts >= since
                        } else {
                            debug!("Event did not have a created_at timestamp. Skipping. {ev:?}");
                            false
                        }
                    });
                    result.extend(new_events);
                },
                Err(e) => {
                    warn!(
                        "Error fetching user event data. {e}. The request was {}?page={page}&per_page={PAGE_LEN}",
                        self.get_events_path()
                    );
                    done = true;
                },
            }
            page += 1;
        }
        result
    }
}
