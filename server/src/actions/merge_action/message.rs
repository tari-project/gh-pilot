use actix::{Addr, Message};
use github_pilot_api::GithubEvent;

use crate::{
    actions::merge_action::action_params::MergeActionParams,
    pub_sub::{ActionResult, PubSubActor},
};

#[derive(Clone)]
pub struct MergeActionMessage {
    pub name: String,
    pub event_name: String,
    pub event: GithubEvent,
    pub params: MergeActionParams,
    pub broadcaster: Option<Addr<PubSubActor>>,
}

impl MergeActionMessage {
    pub fn new<S: Into<String>>(
        name: S,
        event_name: S,
        event: GithubEvent,
        params: MergeActionParams,
        broadcaster: Option<Addr<PubSubActor>>,
    ) -> Self {
        MergeActionMessage {
            name: name.into(),
            event_name: event_name.into(),
            event,
            params,
            broadcaster,
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
    type Result = ActionResult;
}
