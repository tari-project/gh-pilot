//! This module covers the [`Rule`] implementation. You can only create a Rule using the [`RuleBuilder`].
//!
//! For example
//!
//! ```
//!   # use gh_pilot::ghp_api::webhooks::{GithubEvent, PullRequestEvent};
//! use ghp_server::actions::Actions;
//!   # use ghp_server::predicates::PullRequest;
//!   # use ghp_server::rules::RuleBuilder;
//! RuleBuilder::new("my-rule")
//!     .when(PullRequest::opened())
//!     .execute(Actions::closure().with(|name, msg| {
//!         let pr = msg.pull_request().unwrap();
//!         println!("PR {} opened", pr.number());
//!     }).build());
//! ```
//!
//! This module also defines the [`RulePredicate`] trait, which defines behaviour for structs that want to act as rule
//! predicates.
use std::{slice::Iter, sync::Arc};

use crate::{actions::Actions, pub_sub::GithubEventMessage, utilities::timestamp};

/// A [`Rule`] is a combination of predicates, notifications and actions.
/// When the Github Pilot receives an event from the webhook, it will scan all its registered rules. For each rule
/// that is triggered (because one/more of the predicates match the event), all notifications get sent out, and all
/// actions get executed.
pub struct Rule {
    inner_rule: RuleInner,
}

impl Rule {
    /// Return an iterator over this Rule's actions
    pub(crate) fn actions(&self) -> Iter<Arc<Actions>> {
        self.inner_rule.actions.iter()
    }

    /// Determine whether this rule's predicate match against the given github event, returning the first predicate
    /// that matches, or None.
    pub(crate) fn matches(&self, event: &GithubEventMessage) -> Option<Arc<dyn RulePredicate>> {
        self.inner_rule.predicates.iter().find(|&p| p.matches(event)).cloned()
    }

    pub fn name(&self) -> &str {
        self.inner_rule.name.as_str()
    }
}

/// Common fields that get passed between the builder and the rule itself.
struct RuleInner {
    name: String,
    predicates: Vec<Arc<dyn RulePredicate>>,
    actions: Vec<Arc<Actions>>,
}

impl Default for RuleInner {
    fn default() -> Self {
        Self {
            name: RuleInner::time_stamped_name(),
            predicates: Vec::new(),
            actions: Vec::new(),
        }
    }
}

impl RuleInner {
    fn time_stamped_name() -> String {
        format!("rule_{}", timestamp())
    }

    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }
}

/// Define behaviour for structs that want to be defined as Rule predicate conditions.
/// A rule predicate defines a single method, [`matches`], that takes a Github event message and decides whether the
/// predicate is satisfied or not. If so, it returns `true`, otherwise it must return false. This method must not panic
/// or fail.
pub trait RulePredicate: Send + Sync {
    fn matches(&self, event: &GithubEventMessage) -> bool;
}

#[derive(Default)]
pub struct RuleBuilder {
    inner_rule: RuleInner,
}

impl RuleBuilder {
    /// Create a new Github event rule
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            inner_rule: RuleInner::new(name),
        }
    }

    /// Add an event predicate. You can add any number of event predicates. The rule will trigger if _any_ of the
    /// predicates match the event.
    pub fn when(mut self, pred: impl RulePredicate + 'static) -> Self {
        self.inner_rule.predicates.push(Arc::new(pred));
        self
    }

    /// Add an action to the set of actions to fire when the rule matches. You may add any number of actions.
    pub fn execute(mut self, action: Actions) -> Self {
        self.inner_rule.actions.push(Arc::new(action));
        self
    }

    /// Build the rule
    pub fn submit(self) -> Rule {
        Rule {
            inner_rule: self.inner_rule,
        }
    }
}