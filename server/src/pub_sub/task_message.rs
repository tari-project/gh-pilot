use std::sync::Arc;

use actix::Message;
use gh_pilot::ghp_api::webhooks::GithubEvent;

use crate::actions::Action;

#[derive(Clone)]
pub struct TaskExecuteMessage {
    name: String,
    event_name: String,
    event: GithubEvent,
    action: Arc<dyn Action>,
}

impl TaskExecuteMessage {
    pub fn new<S: Into<String>>(name: S, event_name: String, event: GithubEvent, action: Arc<dyn Action>) -> Self {
        Self {
            name: name.into(),
            event_name,
            event,
            action,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn event(&self) -> &GithubEvent {
        &self.event
    }

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    pub fn to_parts(self) -> (String, String, GithubEvent, Arc<dyn Action>) {
        (self.name, self.event_name, self.event, self.action)
    }
}

impl Message for TaskExecuteMessage {
    type Result = ();
}
