//! This module covers the [`Rule`] implementation. You can only create a Rule using the [`RuleBuilder`].
//!
//! For example
//!
//! ```
//!   # use github_pilot_api::models::PullRequestEvent;
//!   # use ghp_server::actions::Actions;
//!   # use ghp_server::predicates::PullRequest;
//!   # use ghp_server::rules::RuleBuilder;
//! RuleBuilder::new("my-rule")
//!     .when(PullRequest::opened())
//!     .execute(
//!         Actions::closure()
//!             .with(|name, msg| {
//!                 let num = msg.unwrap().pull_request().unwrap().number();
//!                 println!("PR {num} opened");
//!             })
//!             .build(),
//!     );
//! ```
//!
//! This module also defines the [`RulePredicate`] trait, which defines behaviour for structs that want to act as rule
//! predicates.
use std::{fmt::Debug, slice::Iter, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{actions::Actions, predicates::Predicate, pub_sub::GithubEventMessage, utilities::timestamp};

pub type ActionVec<'a> = Iter<'a, Arc<Actions>>;

/// A [`Rule`] is a combination of predicates, notifications and actions.
/// When the Github Pilot receives an event from the webhook, it will scan all its registered rules. For each rule
/// that is triggered (because one/more of the predicates match the event), all notifications get sent out, and all
/// actions get executed.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Rule {
    #[serde(flatten)]
    inner_rule: RuleInner,
}

impl Rule {
    /// Return an iterator over this Rule's actions
    pub(crate) fn actions(&self) -> ActionVec {
        self.inner_rule.actions.iter()
    }

    /// Return an iterator over this Rule's "then" actions
    pub(crate) fn then_actions(&self) -> ActionVec {
        self.inner_rule.then_actions.iter()
    }

    /// Determine whether this rule's predicate match against the given github event, returning the first predicate
    /// that matches, or None.
    pub(crate) fn matches(&self, event: &GithubEventMessage) -> Option<Arc<Predicate>> {
        self.inner_rule
            .predicates
            .iter()
            .find(|&p| p.as_rule_predicate().matches(event))
            .cloned()
    }

    pub fn name(&self) -> &str {
        self.inner_rule.name.as_str()
    }
}

/// Common fields that get passed between the builder and the rule itself.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RuleInner {
    name: String,
    #[serde(rename = "when", default)]
    predicates: Vec<Arc<Predicate>>,
    #[serde(rename = "execute", default)]
    actions: Vec<Arc<Actions>>,
    #[serde(rename = "then", default)]
    then_actions: Vec<Arc<Actions>>,
}

impl Default for RuleInner {
    fn default() -> Self {
        Self {
            name: RuleInner::time_stamped_name(),
            predicates: Vec::new(),
            actions: Vec::new(),
            then_actions: Vec::new(),
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
pub trait RulePredicate: Send + Sync + Debug {
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
        self.inner_rule.predicates.push(Arc::new(Predicate::from(pred)));
        self
    }

    /// Add an action to the set of actions to fire when the rule matches. You may add any number of actions.
    pub fn execute(mut self, action: Actions) -> Self {
        self.inner_rule.actions.push(Arc::new(action));
        self
    }

    /// Add an action to execute _only_ if the all primary actions (submitted in [`execute`] succeed.
    pub fn then(mut self, action: Actions) -> Self {
        self.inner_rule.then_actions.push(Arc::new(action));
        self
    }

    /// Build the rule
    pub fn submit(self) -> Rule {
        Rule {
            inner_rule: self.inner_rule,
        }
    }
}
