use actix::Message;
use gh_pilot::ghp_api::webhooks::GithubEvent;
use ghp_api::webhooks::PullRequestEvent;

use crate::{pub_sub::PubSubError, rules::Rule};

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

    pub fn pull_request(&self) -> Option<&PullRequestEvent> {
        match &self.event {
            GithubEvent::PullRequest(pr) => Some(pr),
            _ => None,
        }
    }
}

impl Message for GithubEventMessage {
    type Result = ();
}

pub struct ReplaceRulesMessage {
    pub new_rules: Vec<Rule>,
}

impl Message for ReplaceRulesMessage {
    type Result = Result<usize, PubSubError>;
}

pub struct AddRuleMessage {
    pub new_rule: Rule,
}

impl Message for AddRuleMessage {
    type Result = Result<usize, PubSubError>;
}
