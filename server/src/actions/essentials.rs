use github_pilot_api::webhooks::GithubEvent;
use log::warn;

use crate::actions::{closure_action::ClosureActionParams, github_action::GithubActionParams};

// Implementation notes: --
// Actions must actually be Actors. This lets them have state. Then the action tasks will be messages that are
// sent to the TaskRunner and distributed to the handling actors.
//
// webhook msg -> PubSub, checks rules.
// For each rule triggered,
// for each actor action referenced in `execute`,
// build an action task message
// send it to the relevant action actor
//
// action task message -> ActionTask Actor
// Run task according to specification

#[derive(Clone)]
pub enum Actions {
    // An action that executes an arbitrary closure when the rule is triggered. Use with care. With great power comes
    // great responsibility.
    Closure(Box<ClosureActionParams>),
    Github(Box<GithubActionParams>),
    // An action that does nothing. Generally constructed when a Rule is not well defined
    NullAction,
}

impl Actions {
    pub fn closure() -> ClosureActionBuilder {
        ClosureActionBuilder::new()
    }

    pub fn github() -> GithubActionBuilder {
        GithubActionBuilder::new()
    }

    /// Not sure why you'd want this, but here for completeness :)
    pub fn null() -> Self {
        Self::NullAction
    }
}

/// Helper struct to ergonomically build a new closure action defintion
pub struct ClosureActionBuilder {
    params: Option<ClosureActionParams>,
}

impl ClosureActionBuilder {
    pub fn new() -> Self {
        Self { params: None }
    }

    pub fn with<F: Fn(String, GithubEvent) + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.params = Some(ClosureActionParams::with(f));
        self
    }

    pub fn build(self) -> Actions {
        match self.params {
            None => {
                warn!("A valid closure was not provided to the ClosureActionBuilder. Returning a NullAction");
                Actions::NullAction
            },
            Some(f) => Actions::Closure(Box::new(f)),
        }
    }
}

impl Default for ClosureActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GithubActionBuilder {
    params: Option<GithubActionParams>,
}

impl Default for GithubActionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GithubActionBuilder {
    pub fn new() -> Self {
        Self { params: None }
    }

    pub fn add_label<S: Into<String>>(mut self, label: S) -> Self {
        self.params = Some(GithubActionParams::add_label(label));
        self
    }

    pub fn remove_label<S: Into<String>>(mut self, label: S) -> Self {
        self.params = Some(GithubActionParams::remove_label(label));
        self
    }

    pub fn build(self) -> Actions {
        match self.params {
            None => {
                warn!("No github actions were provided to the GithubActionBuilder. Returning a NullAction");
                Actions::NullAction
            },
            Some(f) => Actions::Github(Box::new(f)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_closure_action_builder() {
        let action = Actions::closure()
            .with(|name, event| {
                println!("{}: {:?}", name, event);
            })
            .build();

        match action {
            Actions::Closure(_) => (),
            _ => panic!("Expected a closure action"),
        }
    }

    #[test]
    fn test_closure_action_builder_no_closure() {
        let action = Actions::closure().build();

        match action {
            Actions::NullAction => (),
            _ => panic!("Expected a null action"),
        }
    }

    #[test]
    fn test_github_action_builder() {
        let action = Actions::github().add_label("test").build();

        match action {
            Actions::Github(_) => (),
            _ => panic!("Expected a github action"),
        }
    }

    #[test]
    fn test_github_action_builder_no_action() {
        let action = Actions::github().build();

        match action {
            Actions::NullAction => (),
            _ => panic!("Expected a null action"),
        }
    }
}
