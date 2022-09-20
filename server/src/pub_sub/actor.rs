use std::sync::{Arc, RwLock};

use actix::{Actor, Context, Handler, Running, SystemService};
use github_pilot_api::webhooks::GithubEvent;
use log::*;

use crate::{
    actions::{
        Actions,
        ClosureActionExecutor,
        ClosureActionMessage,
        ClosureActionParams,
        GithubActionExecutor,
        GithubActionMessage,
        GithubActionParams,
    },
    pub_sub::{GithubEventMessage, PubSubError, ReplaceRulesMessage},
    rules::Rule,
    utilities::timestamp,
};

pub struct PubSubActor {
    rules: RwLock<Vec<Rule>>,
}

impl Default for PubSubActor {
    fn default() -> Self {
        Self::new()
    }
}

impl PubSubActor {
    pub fn new() -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
        }
    }

    fn add_rules(&mut self, clear_first: bool, rules: Vec<Rule>) -> Result<usize, PubSubError> {
        let mut lock_guard = self
            .rules
            .write()
            .map_err(|_| PubSubError::ReplaceRulesError("Pubsub actor write lock is poisoned".to_string()))?;
        if clear_first {
            lock_guard.clear();
        }
        rules.into_iter().for_each(|r| lock_guard.push(r));
        Ok(lock_guard.len())
    }

    fn dispatch_message(
        &self,
        action: Arc<Actions>,
        event_name: String,
        event: GithubEvent,
    ) -> Result<(), PubSubError> {
        match action.as_ref() {
            Actions::Closure(c) => self.dispatch_closure_action(*c.clone(), event_name, event),
            Actions::Github(a) => self.dispatch_github_action(*a.clone(), event_name, event),
            Actions::NullAction => {
                info!("📰 NullAction was dispatched. Doing nothing");
                Ok(())
            },
        }
    }

    fn dispatch_closure_action(
        &self,
        params: ClosureActionParams,
        ev_name: String,
        ev: GithubEvent,
    ) -> Result<(), PubSubError> {
        let name = format!("ClosureAction-{}", timestamp());
        let msg = ClosureActionMessage::new(name, ev_name, ev, params);
        let executor = ClosureActionExecutor::from_registry();
        executor.try_send(msg).map_err(|e| {
            PubSubError::DispatchError(format!("Could not dispatch Closure Action message. {}", e.to_string()))
        })
    }

    fn dispatch_github_action(
        &self,
        params: GithubActionParams,
        ev_name: String,
        ev: GithubEvent,
    ) -> Result<(), PubSubError> {
        let name = format!("GithubAction-{}", timestamp());
        let msg = GithubActionMessage::new(name, ev_name, ev, params);
        let executor = GithubActionExecutor::from_registry();
        executor.try_send(msg).map_err(|e| {
            PubSubError::DispatchError(format!("Could not dispatch Github Action message. {}", e.to_string()))
        })
    }
}

impl Actor for PubSubActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("📰 PubSub actor has started.");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("📰 PubSub actor is stopping.");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("📰 PubSub actor has stopped.");
    }
}

impl Handler<GithubEventMessage> for PubSubActor {
    type Result = ();

    fn handle(&mut self, msg: GithubEventMessage, _ctx: &mut Self::Context) -> Self::Result {
        trace!("📰 PubSub received github event message: {}", msg.name());
        let rules = self.rules.read();
        // FIXME: Return an error properly
        if rules.is_err() {
            warn!("📰 Could not access rules. {}", rules.err().unwrap().to_string());
            return;
        }
        let rules = rules.unwrap();
        let mut rules_matched = 0usize;
        let (event_name, event) = msg.clone().to_parts();
        for rule in rules.iter() {
            // Check if any of the predicates match
            let rule_triggered = rule.matches(&msg);
            // If so, dispatch a tasks to run the actions
            if rule_triggered.is_some() {
                rules_matched += 1;
                info!(
                    "📰 Rule \"{}\" triggered for \"{}\". Running its actions.",
                    rule.name(),
                    msg.name()
                );
                let name = format!("{}-{}.{}", rule.name(), event_name, timestamp());
                for action in rule.actions().cloned() {
                    trace!("📰 Preparing task \"{}\"", name);
                    let dispatch_result = self.dispatch_message(action, event_name.clone(), event.clone());
                    if let Err(e) = dispatch_result {
                        warn!("📰 There was an issue dispatching {}: {}", name, e.to_string());
                    }
                }
            }
        }
        debug!("📰 {} rules matched event \"{}\"", rules_matched, event_name);
    }
}

impl Handler<ReplaceRulesMessage> for PubSubActor {
    type Result = Result<usize, PubSubError>;

    fn handle(&mut self, msg: ReplaceRulesMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.add_rules(true, msg.new_rules)
    }
}
