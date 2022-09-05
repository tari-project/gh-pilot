use actix::{Actor, Context, Handler, Recipient, Running};
use log::{debug, trace, warn};

use crate::pub_sub::{GithubEventMessage};
use crate::pub_sub::task_message::TaskExecuteMessage;
use crate::rules::{Rule};
use crate::utilities::timestamp;

pub struct PubSubActor {
    rules: Vec<Rule>,
    task_runner: Recipient<TaskExecuteMessage>
}

impl PubSubActor {
    pub fn new(task_runner: Recipient<TaskExecuteMessage>) -> Self {
        Self {
            rules: Vec::new(),
            task_runner,
        }
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
        for rule in self.rules.iter() {
            // Check if any of the predicates match
            let rule_triggered = rule.matches(&msg);
            // If so, dispatch a tasks to run the actions
            if rule_triggered.is_some() {
                trace!("Rule {} was triggered. Running its actions.", rule.name());
                let (event_name, event) = msg.clone().to_parts();

                let name = format!("{}-{}.{}", rule.name(), event_name, timestamp());
                for action in rule.actions().cloned() {
                    let task_msg = TaskExecuteMessage::new(name.clone(), event_name.clone(), event.clone(), action);
                    match self.task_runner.try_send(task_msg) {
                        Ok(()) => debug!("Task {} executed happily", name),
                        Err(e) => warn!("Failed to dispatch task {}. {}", name, e.to_string()),
                    }
                }
            }
        }
    }
}
