use std::sync::Arc;

use actix::{Actor, Addr, AsyncContext, Context, Handler, ResponseFuture, Running, SystemService};
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
    events::{BroadcastEventMessage, Subscription},
    pub_sub::{
        messages::ReplaceSubscriptionsMessage,
        ActionResult,
        GithubEventMessage,
        PubSubError,
        ReplaceRulesMessage,
    },
    rules::{ActionVec, Rule},
    utilities::timestamp,
};

pub struct PubSubActor {
    rules: Arc<RwLock<Vec<Rule>>>,
    subscriptions: Arc<RwLock<Vec<Subscription>>>,
    address: Option<Addr<PubSubActor>>,
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
            subscriptions: Arc::new(RwLock::new(Vec::new())),
            address: None,
        }
    }

    async fn dispatch_message(
        action: Arc<Actions>,
        event_name: String,
        event: Option<GithubEvent>,
        addr: Option<Addr<Self>>,
    ) -> Result<ActionResult, PubSubError> {
        match action.as_ref() {
            Actions::AutoMerge(p) => Self::dispatch_merge_action(*p.clone(), event_name, event, addr).await,
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
        ev: Option<GithubEvent>,
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
        ev: Option<GithubEvent>,
        addr: Option<Addr<Self>>,
    ) -> Result<ActionResult, PubSubError> {
        let name = format!("MergeAction-{}", timestamp());
        match ev {
            Some(ev) => {
                let msg = MergeActionMessage::new(name, ev_name, ev, params, addr);
                let executor = MergeExecutor::from_registry();
                executor
                    .send(msg)
                    .await
                    .map_err(|e| PubSubError::DispatchError(format!("Could not dispatch Merge Action message. {}", e)))
            },
            None => {
                let msg = "📰 Cannot perform a Merge Action if the Github Event is not provided. The action will be \
                           abandoned"
                    .to_string();
                debug!("{name}: {msg}");
                Err(PubSubError::DispatchError(msg))
            },
        }
    }

    async fn dispatch_github_action(
        params: GithubActionParams,
        ev_name: String,
        ev: Option<GithubEvent>,
    ) -> Result<ActionResult, PubSubError> {
        let name = format!("GithubAction-{}", timestamp());
        match ev {
            Some(ev) => {
                let msg = GithubActionMessage::new(name, ev_name, ev, params);
                let executor = GithubActionExecutor::from_registry();
                executor
                    .send(msg)
                    .await
                    .map_err(|e| PubSubError::DispatchError(format!("Could not dispatch Github Action message. {}", e)))
            },
            None => {
                let msg = "📰 Cannot perform a Github Action if the Github Event is not provided. The action will be \
                           abandoned"
                    .to_string();
                debug!("{name}: {msg}");
                Err(PubSubError::DispatchError(msg))
            },
        }
    }

    /// Runs all the "Execute" actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`, the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_execute_actions(
        event_name: String,
        event: Option<GithubEvent>,
        rule: &Rule,
        addr: Option<Addr<Self>>,
    ) -> ActionResult {
        let name = format!("{}-{}.execute.{}", rule.name(), event_name, timestamp());
        Self::run_actions(name, event_name, event, rule.actions(), addr).await
    }

    /// Runs all the "then" actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`,
    /// the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_then_actions(
        event_name: String,
        event: Option<GithubEvent>,
        rule: &Rule,
        addr: Option<Addr<Self>>,
    ) -> ActionResult {
        let name = format!("{}-{}.then.{}", rule.name(), event_name, timestamp());
        Self::run_actions(name, event_name, event, rule.then_actions(), addr).await
    }

    /// Runs all the actions attached to this rule. If any action returns `Failed`, `ConditionsNotMet` or
    /// `Indeterminate`, the overall Result is same and the remaining actions are not run.
    ///
    /// Only if all actions return `Success`, will the overall result be `Success`.
    async fn run_actions(
        task: String,
        event_name: String,
        event: Option<GithubEvent>,
        actions: ActionVec<'_>,
        addr: Option<Addr<Self>>,
    ) -> ActionResult {
        for action in actions.cloned() {
            trace!("📰 Dispatching task \"{task}\" on \"{event_name}\"");
            match Self::dispatch_message(action, event_name.clone(), event.clone(), addr.clone()).await {
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

    pub fn broadcast_event(addr: &Option<Addr<Self>>, msg: BroadcastEventMessage) {
        let ev_str = format!("{}", msg.event);
        if let Some(addr) = addr {
            match addr.try_send(msg) {
                Ok(_) => trace!("📰 Successfully broadcast {ev_str} event"),
                Err(e) => warn!("📰 Could not broadcast {ev_str} event: {e}"),
            }
        } else {
            warn!("📰 Could not broadcast event because the PubSub actor address has not been set");
        }
    }

    // note: this private fn cannot call `self` because it is called from an async task.
    async fn evaluate_rules_against_message(
        msg: GithubEventMessage,
        rules: Arc<RwLock<Vec<Rule>>>,
        addr: Option<Addr<Self>>,
    ) {
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
                let result =
                    Self::run_execute_actions(event_name.clone(), Some(event.clone()), rule, addr.clone()).await;
                if matches!(result, ActionResult::Success) {
                    let _ = Self::run_then_actions(event_name.clone(), Some(event.clone()), rule, addr.clone()).await;
                }
                // It may be tempting to broadcast the result here, but don't do it. You will invariably cause an
                // infinite loop and saturate the mailboxes of all the actors.
            }
        }
        debug!("📰 {rules_matched} rules matched event \"{event_name}\"");
    }

    async fn evaluate_subs_against_event(
        msg: BroadcastEventMessage,
        subs: Arc<RwLock<Vec<Subscription>>>,
        addr: Option<Addr<Self>>,
    ) {
        trace!("📰 PubSub received broadcast event: {}", msg.event);
        let subs = subs.read().await;
        let mut subs_matched = 0usize;
        let event_name = format!("{}", msg.event);
        for sub in subs.iter() {
            if sub.matches(&msg.event) && sub.constraints().matches(&msg.event) {
                info!("📰 Subscription \"{}\" triggered for \"{event_name}\".", sub.name());
                subs_matched += 1;
                let task_name = format!("{}.subscription.{event_name}.{}", sub.name(), timestamp());
                let actions = sub.actions();
                let result = Self::run_actions(
                    task_name.clone(),
                    event_name.clone(),
                    msg.github_event.clone(),
                    actions,
                    addr.clone(),
                )
                .await;
                debug!("📰 Subscription task \"{task_name}\" completed with result: {result}");
            }
        }
        debug!("📰 {subs_matched} subscriptions matched event \"{event_name}\"");
    }
}

impl Actor for PubSubActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        self.address = Some(addr);
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
        let addr = self.address.clone();
        let fut = async move {
            Self::evaluate_rules_against_message(msg, copy_of_rules, addr).await;
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

impl Handler<ReplaceSubscriptionsMessage> for PubSubActor {
    type Result = ResponseFuture<usize>;

    fn handle(&mut self, msg: ReplaceSubscriptionsMessage, _ctx: &mut Self::Context) -> Self::Result {
        let subs = Arc::clone(&self.subscriptions);
        let fut = async move {
            let mut my_subs = subs.write().await;
            my_subs.clear();
            msg.new_subscriptions.into_iter().for_each(|r| my_subs.push(r));
            my_subs.len()
        };
        Box::pin(fut)
    }
}

impl Handler<BroadcastEventMessage> for PubSubActor {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: BroadcastEventMessage, _ctx: &mut Self::Context) -> Self::Result {
        let copy_of_subs = self.subscriptions.clone();
        let addr = self.address.clone();
        let fut = async move {
            Self::evaluate_subs_against_event(msg, copy_of_subs, addr).await;
        };
        Box::pin(fut)
    }
}
