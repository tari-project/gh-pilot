//! Github actions (not to be confused with [Github Actions](https://github.com/features/actions).
//!
//! This module defines the [`GithubAction`], which implements the [`crate::Action`] trait so that it can be used in
//! Rules.
//!
//! The action requires a bunch of state, like login credentials to the Github API etc. We don't want to clutter the
//! UX with providing all that nonsense when defining rules, so this module also defines a factory struct, [`Github`]
//! which gives you that buttery smooth API for writing lovely rules like
//!
//! ```
//! # use ghp_server::actions::Actions;
//! # use ghp_server::predicates::PullRequest;
//! # use ghp_server::rules::RuleBuilder;
//! RuleBuilder::new("MyRule")
//!     .when(PullRequest::opened())
//!     .execute(Actions::github().add_label("Bug").build()) // It just works like magic!
//!     .submit();
//! ```

use actix::{Actor, Context, Handler, Message, ResponseFuture, Running, Supervised, SystemService};
use ghp_api::webhooks::GithubEvent;
use log::{debug, warn};

#[derive(Clone)]
pub enum GithubActionParams {
    // Adds a label to the PR or Issue (context dependent)
    AddLabel { label: String },
    // Removes a label from the PR or Issue (context dependent)
    RemoveLabel { label: String },
}

impl GithubActionParams {
    pub fn add_label<S: Into<String>>(label: S) -> Self {
        GithubActionParams::AddLabel { label: label.into() }
    }

    pub fn remove_label<S: Into<String>>(label: S) -> Self {
        GithubActionParams::RemoveLabel { label: label.into() }
    }
}

#[derive(Clone)]
pub struct GithubActionMessage {
    name: String,
    event_name: String,
    event: GithubEvent,
    params: GithubActionParams,
}

impl GithubActionMessage {
    pub fn new<S: Into<String>>(name: S, event_name: S, event: GithubEvent, params: GithubActionParams) -> Self {
        GithubActionMessage {
            name: name.into(),
            event_name: event_name.into(),
            event,
            params,
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

    pub fn params(&self) -> &GithubActionParams {
        &self.params
    }
}

impl Message for GithubActionMessage {
    type Result = ();
}

#[derive(Default)]
pub struct GithubActionExecutor {}

impl GithubActionExecutor {}

impl Supervised for GithubActionExecutor {}

impl SystemService for GithubActionExecutor {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        debug!("Github Action Executor service has started");
    }
}

impl Actor for GithubActionExecutor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("Github Action Executor has started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        println!("Github Action Executor is stopping");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("Github Action Executor has stopped");
    }
}

impl Handler<GithubActionMessage> for GithubActionExecutor {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: GithubActionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let fut = async move {
            match (msg.event(), msg.params()) {
                (GithubEvent::Issues(event), GithubActionParams::AddLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let issue_number = event.number();
                    debug!("Adding label {} to issue {}/{}#{}", label, owner, repo, issue_number);
                    // let res = ghp_api::issues::add_label(owner, repo, issue_number, label).await;
                    // if let Err(e) = res {
                    //     warn!("Failed to add label to issue: {}", e);
                    // }
                },
                (GithubEvent::Issues(event), GithubActionParams::RemoveLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let issue_number = event.number();
                    debug!(
                        "Removing label {} from issue {}/{}#{}",
                        label, owner, repo, issue_number
                    );
                    // let res = ghp_api::issues::remove_label(owner, repo, issue_number, label).await;
                    // if let Err(e) = res {
                    //     warn!("Failed to remove label from issue: {}", e);
                    // }
                },
                (GithubEvent::PullRequest(event), GithubActionParams::AddLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let pr_number = event.number();
                    debug!("Adding label {} to PR {}/{}#{}", label, owner, repo, pr_number);
                    // let res = ghp_api::pulls::add_label(owner, repo, pr_number, label).await;
                    // if let Err(e) = res {
                    //     warn!("Failed to add label to PR: {}", e);
                    // }
                },
                (GithubEvent::PullRequest(event), GithubActionParams::RemoveLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let pr_number = event.number();
                    debug!("Removing label {} from PR {}/{}#{}", label, owner, repo, pr_number);
                    // let res = ghp_api::pulls::remove_label(owner, repo, pr_number, label).await;
                    // if let Err(e) = res {
                    //     warn!("Failed to remove label from PR: {}", e);
                    // }
                },
                _ => {
                    warn!("Unimplemented event type for Github Action: {}", msg.event_name());
                },
            }
        };
        Box::pin(fut)
    }
}
