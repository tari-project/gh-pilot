use actix::Message;
use gh_pilot::ghp_api::webhooks::GithubEvent;

pub struct GithubEventMessage {
    name: String,
    event: GithubEvent,
}

impl GithubEventMessage {
    pub fn new(name: &str, event: GithubEvent) -> Self {
        Self {
            name: name.to_string(),
            event,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn event(&self) -> &GithubEvent {
        &self.event
    }

    pub fn to_parts(self) -> (String, GithubEvent) {
        (self.name, self.event)
    }
}

impl Message for GithubEventMessage {
    type Result = ();
}
