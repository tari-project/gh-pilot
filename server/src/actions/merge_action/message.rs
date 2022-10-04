use actix::Message;
use github_pilot_api::webhooks::GithubEvent;

use crate::actions::merge_action::action_params::MergeActionParams;

#[derive(Clone)]
pub struct MergeActionMessage {
    pub name: String,
    pub event_name: String,
    pub event: GithubEvent,
    pub params: MergeActionParams,
}

impl MergeActionMessage {
    pub fn new<S: Into<String>>(name: S, event_name: S, event: GithubEvent, params: MergeActionParams) -> Self {
        MergeActionMessage {
            name: name.into(),
            event_name: event_name.into(),
            event,
            params,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    pub fn event(&self) -> &GithubEvent {
        &self.event
    }

    pub fn params(&self) -> &MergeActionParams {
        &self.params
    }
}

impl Message for MergeActionMessage {
    type Result = ();
}