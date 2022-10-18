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
use github_pilot_api::{
    models::{IssuesEvent, PullRequestEvent},
    provider_traits::IssueProvider,
    wrappers::IssueId,
    GithubEvent,
    GithubProvider,
};
use log::*;

use crate::pub_sub::ActionResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GithubActionParams {
    // Adds a label to the PR or Issue (context dependent)
    AddLabel { label: String },
    // Removes a label from the PR or Issue (context dependent)
    RemoveLabel { label: String },
    // Adds or removes the `merge-conflict` label depending on whether the PR has merge conflicts
    CheckConflicts,
}

impl GithubActionParams {
    pub fn add_label<S: Into<String>>(label: S) -> Self {
        GithubActionParams::AddLabel { label: label.into() }
    }

    pub fn remove_label<S: Into<String>>(label: S) -> Self {
        GithubActionParams::RemoveLabel { label: label.into() }
    }

    pub fn check_conflicts() -> Self {
        GithubActionParams::CheckConflicts
    }
}

#[derive(Clone, Debug)]
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
    type Result = ActionResult;
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
        debug!("🐙 Github Action Executor service has started");
    }
}

impl Actor for GithubActionExecutor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("🐙 Github Action Executor has started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("🐙 Github Action Executor is stopping");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("🐙 Github Action Executor has stopped");
    }
}

impl Handler<GithubActionMessage> for GithubActionExecutor {
    type Result = ResponseFuture<ActionResult>;

    fn handle(&mut self, msg: GithubActionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let provider = Arc::clone(&self.provider);

        let fut = async move {
            if let Some(id) = msg.event.related_pull_request() {
                match msg.params {
                    GithubActionParams::AddLabel { label } => {
                        return Self::add_label_to_pr(&provider, &id, &label).await
                    },
                    GithubActionParams::RemoveLabel { label } => {
                        return Self::remove_label_from_pr(&provider, &id, &label).await
                    },
                    _ => {}, // no-op
                }
            }
            match (msg.event(), msg.params()) {
                (GithubEvent::Issues(event), GithubActionParams::AddLabel { label }) => {
                    Self::add_label_to_issue(&provider, event, label).await
                },
                (GithubEvent::Issues(event), GithubActionParams::RemoveLabel { label }) => {
                    Self::remove_label_from_issue(&provider, event, label).await
                },
                (GithubEvent::PullRequest(event), GithubActionParams::CheckConflicts) => {
                    Self::check_and_label_merge_conflicts(provider, event).await
                },
                _ => {
                    warn!("🐙 Unimplemented event type for Github Action: {}", msg.event_name());
                    debug!(
                        "🐙 Event parameters resulting in mismatch by GithubAction: {:?}, {:?}",
                        msg.event(),
                        msg.params()
                    );
                    ActionResult::Indeterminate
                },
            }
        };
        Box::pin(fut)
    }
}

impl GithubActionExecutor {
    async fn add_label_to_issue(provider: &Arc<GithubProvider>, event: &IssuesEvent, label: &String) -> ActionResult {
        let repo = event.repo();
        let owner = event.owner();
        let issue_number = event.number();
        debug!(
            "🐙🏷 Adding label {} to issue {}/{}#{}",
            label, owner, repo, issue_number
        );
        let req = IssueId::new(owner, repo, issue_number);
        let res = provider.add_label(&req, label).await;
        ActionResult::from_result(
            res,
            || info!("🐙🏷 Added label {label} to issue {req}"),
            |e| warn!("🐙🏷 Failed to add label to issue {req}: {e}"),
        )
    }

    async fn remove_label_from_issue(
        provider: &Arc<GithubProvider>,
        event: &IssuesEvent,
        label: &String,
    ) -> ActionResult {
        let id = IssueId::new(event.owner(), event.repo(), event.number());
        debug!("🐙🏷 Removing label {label} from issue {id}");
        match provider.remove_label(&id, label, false).await {
            Ok(true) => {
                info!("🐙🏷 '{label}' removed from issue {id}");
                ActionResult::Success
            },
            Ok(false) => {
                info!("🐙🏷 '{label}' not found on issue {id}, so nothing to remove 😏");
                ActionResult::Success
            },
            Err(e) => {
                warn!("🐙🏷 '{label}' removal failed on issue {id}. {e}");
                ActionResult::Failed
            },
        }
    }

    async fn add_label_to_pr(provider: &Arc<GithubProvider>, id: &IssueId, label: &String) -> ActionResult {
        debug!("🐙🏷 Adding label {label} to PR {id}");
        let res = provider.add_label(id, label).await;
        ActionResult::from_result(
            res,
            || info!("🐙🏷 Added label {label} to PR {id}"),
            |e| warn!("🐙🏷 Failed to add label to PR {id}: {e}"),
        )
    }

    async fn remove_label_from_pr(provider: &Arc<GithubProvider>, id: &IssueId, label: &String) -> ActionResult {
        debug!("🐙🏷 Removing label [{label}] from PR {id}");
        match provider.remove_label(id, label, false).await {
            Ok(true) => {
                info!("🐙🏷 [{label}] removed from PR {id}");
                ActionResult::Success
            },
            Ok(false) => {
                info!("🐙🏷 [{label}] not found on PR {id}, so nothing to remove 😏");
                ActionResult::Success
            },
            Err(e) => {
                warn!("🐙🏷 '{label}' removal failed on PR: {id}. {e}");
                ActionResult::Failed
            },
        }
    }

    async fn check_and_label_merge_conflicts(provider: Arc<GithubProvider>, event: &PullRequestEvent) -> ActionResult {
        let id = IssueId::new(event.owner(), event.repo(), event.number());
        debug!("🐙🤺 Checking merge conflict status for PR {id}");
        let conflict_label = "P-conflicts";
        let pr = event.pull_request();
        let res = if pr.has_merge_conflicts() {
            info!("🐙🤺 PR {id} has merge conflicts. Adding label [{conflict_label}]");
            provider.add_label(&id, conflict_label).await.map(|_| ())
        } else {
            match provider.remove_label(&id, conflict_label, true).await {
                Ok(true) => {
                    info!("🐙🤺 Merge conflicts resolved on PR {id}. Label removed.");
                    Ok(())
                },
                Ok(false) => {
                    trace!(
                        "🐙🤺 Merge conflict label wasn't attached to the PR {id}. There likely weren't merge \
                         conflicts in the first place"
                    );
                    Ok(())
                },
                Err(e) => Err(e),
            }
        };
        ActionResult::from_result(
            res,
            || debug!("🐙🤺 Merge conflict status check complete for PR {id}"),
            |e| warn!("🐙🤺 Failed to check merge conflict status for PR {id}: {e}"),
        )
    }
}
