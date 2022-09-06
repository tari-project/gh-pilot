use std::sync::RwLock;

use actix::{Actor, Context, Handler, Recipient, Running};
use log::*;

use crate::{
    pub_sub::{task_message::TaskExecuteMessage, GithubEventMessage, PubSubError, ReplaceRulesMessage},
    rules::Rule,
    utilities::timestamp,
};

pub struct PubSubActor {
    rules: RwLock<Vec<Rule>>,
    task_runner: Recipient<TaskExecuteMessage>,
}

impl PubSubActor {
    pub fn new(task_runner: Recipient<TaskExecuteMessage>) -> Self {
        Self {
            rules: RwLock::new(Vec::new()),
            task_runner,
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
}

impl Actor for PubSubActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("PubSub actor has started.");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("PubSub actor is stopping.");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("PubSub actor has stopped.");
    }
}

impl Handler<GithubEventMessage> for PubSubActor {
    type Result = ();

    fn handle(&mut self, msg: GithubEventMessage, _ctx: &mut Self::Context) -> Self::Result {
        trace!("PubSub received github event message: {}", msg.name());
        let rules = self.rules.read();
        // FIXME: Return an error properly
        if rules.is_err() {
            warn!("Could not access rules. {}", rules.err().unwrap().to_string());
            return;
        }
        let rules = rules.unwrap();
        for rule in rules.iter() {
            // Check if any of the predicates match
            let rule_triggered = rule.matches(&msg);
            // If so, dispatch a tasks to run the actions
            if rule_triggered.is_some() {
                info!(
                    "Rule \"{}\" triggered for \"{}\". Running its actions.",
                    rule.name(),
                    msg.name()
                );
                let (event_name, event) = msg.clone().to_parts();

                let name = format!("{}-{}.{}", rule.name(), event_name, timestamp());
                for action in rule.actions().cloned() {
                    let task_msg = TaskExecuteMessage::new(name.clone(), event_name.clone(), event.clone(), action);
                    match self.task_runner.try_send(task_msg) {
                        Ok(()) => trace!("Task \"{}\" dispatched successfully", name),
                        Err(e) => warn!("Failed to dispatch task \"{}\". {}", name, e.to_string()),
                    }
                }
            }
        }
    }
}

impl Handler<ReplaceRulesMessage> for PubSubActor {
    type Result = Result<usize, PubSubError>;

    fn handle(&mut self, msg: ReplaceRulesMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.add_rules(true, msg.new_rules)
    }
}
