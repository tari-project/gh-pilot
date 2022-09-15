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

use std::sync::Arc;

use actix::{Actor, Context, Handler, Message, ResponseFuture, Running, Supervised, SystemService};
use gh_pilot::{data_provider::IssueProvider, models::IssueId, GithubProvider};
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

pub struct GithubActionExecutor {
    provider: Arc<GithubProvider>,
}

impl Default for GithubActionExecutor {
    fn default() -> Self {
        // Will pull credentials from envars if possible
        let github_provider = GithubProvider::default();
        Self {
            provider: Arc::new(github_provider),
        }
    }
}

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
        let provider = Arc::clone(&self.provider);
        let fut = async move {
            match (msg.event(), msg.params()) {
                (GithubEvent::Issues(event), GithubActionParams::AddLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let issue_number = event.number();
                    debug!("Adding label {} to issue {}/{}#{}", label, owner, repo, issue_number);
                    let req = IssueId::new(owner, repo, issue_number);
                    let res = provider.add_label(&req, label).await;
                    if let Err(e) = res {
                        warn!("Failed to add label to issue: {}", e.to_string());
                    }
                },
                (GithubEvent::Issues(event), GithubActionParams::RemoveLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let issue_number = event.number();
                    debug!(
                        "Removing label {} from issue {}/{}#{}",
                        label, owner, repo, issue_number
                    );
                    let req = IssueId::new(owner, repo, issue_number);
                    let res = provider.remove_label(&req, label).await;
                    if let Err(e) = res {
                        warn!("Failed to remove label to issue: {}", e.to_string());
                    }
                },
                (GithubEvent::PullRequest(event), GithubActionParams::AddLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let pr_number = event.number();
                    debug!("Adding label {} to PR {}/{}#{}", label, owner, repo, pr_number);
                    let req = IssueId::new(owner, repo, pr_number);
                    let res = provider.add_label(&req, label).await;
                    if let Err(e) = res {
                        warn!("Failed to add label to PR: {}", e.to_string());
                    }
                },
                (GithubEvent::PullRequest(event), GithubActionParams::RemoveLabel { label }) => {
                    let repo = event.repo();
                    let owner = event.owner();
                    let pr_number = event.number();
                    debug!("Removing label {} from PR {}/{}#{}", label, owner, repo, pr_number);
                    let req = IssueId::new(owner, repo, pr_number);
                    let res = provider.remove_label(&req, label).await;
                    if let Err(e) = res {
                        warn!("Failed to remove label from PR: {}", e.to_string());
                    }
                },
                _ => {
                    warn!("Unimplemented event type for Github Action: {}", msg.event_name());
                },
            }
        };
        Box::pin(fut)
    }
}
