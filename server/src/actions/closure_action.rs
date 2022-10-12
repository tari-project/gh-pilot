use std::sync::Arc;

use actix::{Actor, Context, Handler, Message, ResponseFuture, Running, Supervised, SystemService};
use github_pilot_api::webhook_events::GithubEvent;
use log::*;

type ClosureActionFn = Arc<dyn Fn(String, GithubEvent) + Send + Sync>;

/// An action implementation that wraps a closure
#[derive(Clone)]
pub struct ClosureActionParams {
    function: ClosureActionFn,
}

impl ClosureActionParams {
    pub fn with<F: Fn(String, GithubEvent) + Send + Sync + 'static>(f: F) -> Self {
        Self { function: Arc::new(f) }
    }
}

#[derive(Clone)]
pub struct ClosureActionMessage {
    name: String,
    event_name: String,
    event: GithubEvent,
    params: ClosureActionParams,
}

impl ClosureActionMessage {
    pub fn new<S: Into<String>>(name: S, event_name: String, event: GithubEvent, params: ClosureActionParams) -> Self {
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

    pub fn event(&self) -> &GithubEvent {
        &self.event
    }

    pub fn event_name(&self) -> &str {
        self.event_name.as_str()
    }

    pub fn to_parts(self) -> (String, String, GithubEvent, ClosureActionParams) {
        (self.name, self.event_name, self.event, self.params)
    }
}

impl Message for ClosureActionMessage {
    type Result = ();
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
    type Result = ResponseFuture<()>;

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
            match result {
                Ok(()) => debug!("📝 Closure Task completely happily."),
                Err(e) => debug!("📝 Closure task wasn't happy. {}", e.to_string()),
            }
            debug!("📝 Completed execution of task \"{}\"", name);
        })
    }
}
