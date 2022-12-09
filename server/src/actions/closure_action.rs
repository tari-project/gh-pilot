use std::{
    any::Any,
    fmt::{Debug, Formatter},
    sync::Arc,
};

use actix::{Actor, Context, Handler, Message, ResponseFuture, Running, Supervised, SystemService};
use github_pilot_api::GithubEvent;
use log::*;
use serde::{Deserialize, Serialize};

use crate::pub_sub::ActionResult;

type ClosureActionFn = Arc<dyn Fn(String, Option<GithubEvent>) + Send + Sync>;

/// An action implementation that wraps a closure
#[derive(Clone, Serialize, Deserialize)]
pub struct ClosureActionParams {
    #[serde(skip, default = "default_closure")]
    function: ClosureActionFn,
}

impl ClosureActionParams {
    pub fn with<F: Fn(String, Option<GithubEvent>) + Send + Sync + 'static>(f: F) -> Self {
        Self { function: Arc::new(f) }
    }
}

impl PartialEq for ClosureActionParams {
    fn eq(&self, other: &Self) -> bool {
        self.function.type_id() == other.function.type_id()
    }
}

impl Eq for ClosureActionParams {}

impl Debug for ClosureActionParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("function: <closure>")
    }
}

fn default_closure() -> ClosureActionFn {
    Arc::new(|_, _| {})
}

#[derive(Clone)]
pub struct ClosureActionMessage {
    name: String,
    event_name: String,
    event: Option<GithubEvent>,
    params: ClosureActionParams,
}

impl ClosureActionMessage {
    pub fn new<S: Into<String>>(
        name: S,
        event_name: String,
        event: Option<GithubEvent>,
        params: ClosureActionParams,
    ) -> Self {
        Self {
            name: name.into(),
            event_name,
            event,
            params,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn event(&self) -> Option<&GithubEvent> {
        self.event.as_ref()
    }

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    pub fn to_parts(self) -> (String, String, Option<GithubEvent>, ClosureActionParams) {
        (self.name, self.event_name, self.event, self.params)
    }
}

impl Message for ClosureActionMessage {
    type Result = ActionResult;
}

#[derive(Default)]
pub struct ClosureActionExecutor {}

impl ClosureActionExecutor {}

impl Supervised for ClosureActionExecutor {}

impl SystemService for ClosureActionExecutor {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        debug!("📝 Closure Action Service is running.");
    }
}

impl Actor for ClosureActionExecutor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("📝 Closure Action actor has started.");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("📝 Closure Action actor is stopping.");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("📝 Closure Action actor has stopped.");
    }
}

impl Handler<ClosureActionMessage> for ClosureActionExecutor {
    type Result = ResponseFuture<ActionResult>;

    fn handle(&mut self, msg: ClosureActionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let (name, event_name, event, action) = msg.to_parts();
        debug!("📝 Starting task \"{}\" for \"{}\"", name, event_name);
        Box::pin(async move {
            debug!("📝 Running closure action");
            let f = action.function;
            let result = tokio::task::spawn_blocking(move || {
                f(event_name, event);
            })
            .await;
            debug!("📝 Completed execution of task \"{}\"", name);
            ActionResult::from_result(
                result,
                || debug!("📝 Closure Task completely happily."),
                |e| debug!("📝 Closure task wasn't happy. {e}"),
            )
        })
    }
}
