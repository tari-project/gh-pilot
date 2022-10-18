use std::sync::Arc;

use crate::{
    actions::Actions,
    events::{constraints::EventConstraints, BroadcastEvent, Event, EventConstraint},
    rules::ActionVec,
};

pub struct Subscription {
    sub: SubscriptionInner,
}

pub struct SubscriptionBuilder {
    sub: SubscriptionInner,
}

#[derive(Default)]
struct SubscriptionInner {
    name: String,
    event: Event,
    constraints: EventConstraints,
    actions: Vec<Arc<Actions>>,
}

impl SubscriptionBuilder {
    /// Start a new event subscription definition
    pub fn on<S: Into<String>>(name: S, event: Event) -> Self {
        let sub = SubscriptionInner {
            name: name.into(),
            event,
            ..SubscriptionInner::default()
        };
        Self { sub }
    }

    /// Add a constraint to limit the instances of `Event` that will trigger this subscription's actions
    pub fn and(mut self, constraint: impl EventConstraint + 'static) -> Self {
        self.sub.constraints.add(constraint);
        self
    }

    /// Add an action to execute _only_ if all the constraints are satisfied.
    pub fn then(mut self, action: Actions) -> Self {
        self.sub.actions.push(Arc::new(action));
        self
    }

    /// Completes the subscription definition and returns a `Subscription` instance
    pub fn submit(self) -> Subscription {
        Subscription { sub: self.sub }
    }
}

impl Subscription {
    /// Get the name of this subscription
    pub fn name(&self) -> &str {
        self.sub.name.as_str()
    }

    /// Get the event that triggers this subscription
    pub fn event(&self) -> &Event {
        &self.sub.event
    }

    /// Get the constraints that must be satisfied for this subscription's actions to be executed
    pub fn constraints(&self) -> &EventConstraints {
        &self.sub.constraints
    }

    /// Get the actions that will be executed if all the constraints are satisfied
    pub fn actions(&self) -> ActionVec {
        self.sub.actions.iter()
    }

    pub fn matches(&self, event: &BroadcastEvent) -> bool {
        match (self.sub.event, event) {
            (Event::ReviewsNeeded, BroadcastEvent::ReviewsNeeded(_)) => true,
            (Event::ReviewsThresholdReached, BroadcastEvent::ReviewsThresholdReached) => true,
            (Event::AcksNeeded, BroadcastEvent::AcksNeeded(_)) => true,
            (Event::AcksThresholdReached, BroadcastEvent::AcksThresholdReached) => true,
            (Event::ChangesRequested, BroadcastEvent::ChangesRequested) => true,
            (Event::Default, _) => false,
            _ => false,
        }
    }
}
