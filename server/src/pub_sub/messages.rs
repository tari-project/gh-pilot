use actix::Message;
use github_pilot_api::GithubEvent;

use crate::{events::Subscription, rules::Rule};

#[derive(Debug, Clone)]
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

pub struct ReplaceRulesMessage {
    pub new_rules: Vec<Rule>,
}

impl Message for ReplaceRulesMessage {
    type Result = usize;
}

pub struct AddRuleMessage {
    pub new_rule: Rule,
}

impl Message for AddRuleMessage {
    type Result = usize;
}

pub struct ReplaceSubscriptionsMessage {
    pub new_subscriptions: Vec<Subscription>,
}

impl Message for ReplaceSubscriptionsMessage {
    type Result = usize;
}
