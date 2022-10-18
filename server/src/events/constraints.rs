use crate::events::BroadcastEvent;

pub trait EventConstraint: Send + Sync {
    fn matches(&self, event: &BroadcastEvent) -> bool;
}

#[derive(Default)]
pub struct EventConstraints {
    constraints: Vec<Box<dyn EventConstraint>>,
}

impl EventConstraints {
    /// Create a new, empty set of event constraints
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new constraint to the set
    pub fn add(&mut self, constraint: impl EventConstraint + 'static) {
        self.constraints.push(Box::new(constraint));
    }

    /// Check whether the set of constraints is satisfied by the given event. `matches` returns `true` if the
    /// constraints are satisfied, indicating that the response to the event should be triggered.
    pub fn matches(&self, event: &BroadcastEvent) -> bool {
        self.constraints.iter().all(|c| c.matches(event))
    }
}
