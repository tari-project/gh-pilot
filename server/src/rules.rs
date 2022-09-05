use crate::pub_sub::GithubEventMessage;

pub struct RuleInner<F> {
    predicates: Vec<Box<dyn RulePredicate>>,
    actions: Vec<F>,
}

impl<F> Default for RuleInner<F> {
    fn default() -> Self {
        Self {
            predicates: Vec::new(),
            actions: Vec::new(),
        }
    }
}

pub struct Notification;

pub trait RulePredicate {
    fn matches(&self, event: &GithubEventMessage) -> bool;
}

pub struct RuleBuilder<F> {
    inner_rule: RuleInner<F>,
}

impl<F> RuleBuilder<F> {
    /// Create a new Github event rule
    pub fn new() -> Self {
        Self {
            inner_rule: Default::default(),
        }
    }

    /// Add an event predicate. You can add any number of event predicates. The rule will trigger if _any_ of the
    /// predicates match the event.
    pub fn when(mut self, pred: impl RulePredicate + 'static) -> Self {
        self.inner_rule.predicates.push(Box::new(pred));
        self
    }
}

impl<F> RuleBuilder<F>
where F: Fn(&dyn RulePredicate, &GithubEventMessage) + Send + Sync
{
    /// Add an action to the set of actions to fire when the rule matches. You may add any number of actions.
    pub fn execute(mut self, action: F) -> Self {
        self.inner_rule.actions.push(action);
        self
    }

    /// Build the rule
    pub fn submit(self) -> Rule<F> {
        Rule {
            inner_rule: self.inner_rule,
        }
    }
}

pub struct Rule<F> {
    inner_rule: RuleInner<F>,
}

impl<F> Rule<F>
where F: Fn(&dyn RulePredicate, &GithubEventMessage) + Send + Sync
{
    /// Run this rule's actions concurrently, each in their own blocking task
    pub fn run(&self) {}

    /// Determine whether this rule's predicate match against the given github event
    pub fn matches(&self, event: &GithubEventMessage) -> bool {
        self.inner_rule.predicates.iter().any(|p| p.matches(event))
    }
}
