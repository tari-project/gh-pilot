use std::sync::Arc;

use actix::{Actor, Context, Handler, ResponseFuture, Running, SystemService};
use github_pilot_api::GithubEvent;
use log::*;
use tokio::sync::RwLock;

use crate::{
    actions::{
        Actions,
        ClosureActionExecutor,
        ClosureActionMessage,
        ClosureActionParams,
        GithubActionExecutor,
        GithubActionMessage,
        GithubActionParams,
        MergeActionMessage,
        MergeActionParams,
        MergeExecutor,
    },
    pub_sub::{ActionResult, GithubEventMessage, PubSubError, ReplaceRulesMessage},
    rules::{ActionVec, Rule},
    utilities::timestamp,
};

pub struct PubSubActor {
    rules: Arc<RwLock<Vec<Rule>>>,
}

impl Default for PubSubActor {
    fn default() -> Self {
        Self::new()
    }
}

impl PubSubActor {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn dispatch_message(
        action: Arc<Actions>,
        event_name: String,
        event: GithubEvent,
    ) -> Result<ActionResult, PubSubError> {
        match action.as_ref() {
            Actions::AutoMerge(p) => Self::dispatch_merge_action(*p.clone(), event_name, event).await,
            Actions::Closure(c) => Self::dispatch_closure_action(*c.clone(), event_name, event).await,
            Actions::Github(a) => Self::dispatch_github_action(*a.clone(), event_name, event).await,
            Actions::NullAction => {
                info!("📰 NullAction was dispatched. Doing nothing");
                Ok(ActionResult::Success)
            },
        }
    }

    async fn dispatch_closure_action(
        params: ClosureActionParams,
        ev_name: String,
        ev: GithubEvent,
    ) -> Result<ActionResult, PubSubError> {
        let name = format!("ClosureAction-{}", timestamp());
        let msg = ClosureActionMessage::new(name, ev_name, ev, params);
        let executor = ClosureActionExecutor::from_registry();
        executor
            .send(msg)
            .await
            .map_err(|e| PubSubError::DispatchError(format!("Could not dispatch Closure Action message. {}", e)))
    }

    async fn dispatch_merge_action(
        params: MergeActionParams,
        ev_name: String,
        ev: GithubEvent,
    ) -> Result<ActionResult, PubSubError> {
        let name = format!("MergeAction-{}", timestamp());
        let msg = MergeActionMessage::new(name, ev_name, ev, params);
        let executor = MergeExecutor::from_registry();
        executor
            .send(msg)
            .await
            .map_err(|e| PubSubError::DispatchError(format!("Could not dispatch Merge Action message. {}", e)))
    }

    async fn dispatch_github_action(
        params: GithubActionParams,
        ev_name: String,
        ev: GithubEvent,
    ) -> Result<ActionResult, PubSubError> {
        let name = format!("GithubAction-{}", timestamp());
        let msg = GithubActionMessage::new(name, ev_name, ev, params);
        let executor = GithubActionExecutor::from_registry();
        executor
            .send(msg)
            .await
            .map_err(|e| PubSubError::DispatchError(format!("Could not dispatch Github Action message. {}", e)))
    }

    /// Runs all the "Execute" actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`, the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_execute_actions(event_name: String, event: GithubEvent, rule: &Rule) -> ActionResult {
        let name = format!("{}-{}.execute.{}", rule.name(), event_name, timestamp());
        Self::run_actions(name, event_name, event, rule.actions()).await
    }

    /// Runs all the "then" actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`,
    /// the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_then_actions(event_name: String, event: GithubEvent, rule: &Rule) -> ActionResult {
        let name = format!("{}-{}.then.{}", rule.name(), event_name, timestamp());
        Self::run_actions(name, event_name, event, rule.then_actions()).await
    }

    /// Runs all the actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`, the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_actions(task: String, event_name: String, event: GithubEvent, actions: ActionVec<'_>) -> ActionResult {
        for action in actions.cloned() {
            trace!("📰 Dispatching task \"{task}\" on \"{event_name}\"");
            match Self::dispatch_message(action, event_name.clone(), event.clone()).await {
                Ok(ActionResult::Success) => {
                    debug!("📰 Task \"{task}\" on \"{event_name}\" completed successfully")
                },
                Ok(ActionResult::ConditionsNotMet) => {
                    debug!("📰 Task \"{task}\" on \"{event_name}\" was not executed because conditions were not met");
                    return ActionResult::ConditionsNotMet;
                },
                Ok(ActionResult::Failed) => {
                    debug!("📰 Task \"{task}\" on \"{event_name}\" failed");
                    return ActionResult::Failed;
                },
                Ok(ActionResult::Indeterminate) => {
                    debug!("📰 Task \"{task}\" on \"{event_name}\" was indeterminate");
                    return ActionResult::Indeterminate;
                },
                Err(e) => {
                    warn!("📰 There was an issue dispatching {task}: {e}");
                    return ActionResult::Failed;
                },
            }
        }
        debug!("📰 All actions for Task \"{task}\" on \"{event_name}\" completed successfully");
        ActionResult::Success
    }

    async fn broadcast_result(_result: ActionResult, _event: GithubEvent) {
        // TODO
        // create a new InternalEventMessage
        // send the message to the InternalEventActor
        // Log the result
    }

    // note: this private fn cannot call `self` because it is called from an async task.
    async fn evaluate_rules_against_message(msg: GithubEventMessage, rules: Arc<RwLock<Vec<Rule>>>) {
        trace!("📰 PubSub received github event message: {}", msg.name());
        let rules = rules.read().await;
        let mut rules_matched = 0usize;
        let (event_name, event) = msg.clone().to_parts();
        for rule in rules.iter() {
            // Check if any of the predicates match
            let rule_triggered = rule.matches(&msg);
            // If so, dispatch a tasks to run the actions
            if rule_triggered.is_some() {
                rules_matched += 1;
                info!("📰 Rule \"{}\" triggered for \"{}\".", rule.name(), msg.name());
                let result = Self::run_execute_actions(event_name.clone(), event.clone(), rule).await;
                if matches!(result, ActionResult::Success) {
                    let _ = Self::run_then_actions(event_name.clone(), event.clone(), rule).await;
                }
                Self::broadcast_result(result, event.clone()).await;
            }
        }
        debug!("📰 {rules_matched} rules matched event \"{event_name}\"");
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
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: GithubEventMessage, _ctx: &mut Self::Context) -> Self::Result {
        let copy_of_rules = self.rules.clone();
        let fut = async move {
            Self::evaluate_rules_against_message(msg, copy_of_rules).await;
        };
        Box::pin(fut)
    }
}

impl Handler<ReplaceRulesMessage> for PubSubActor {
    type Result = ResponseFuture<usize>;

    fn handle(&mut self, msg: ReplaceRulesMessage, _ctx: &mut Self::Context) -> Self::Result {
        let rules = Arc::clone(&self.rules);
        let fut = async move {
            let mut my_rules = rules.write().await;
            my_rules.clear();
            msg.new_rules.into_iter().for_each(|r| my_rules.push(r));
            my_rules.len()
        };
        Box::pin(fut)
    }
}
