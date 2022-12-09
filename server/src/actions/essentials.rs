use github_pilot_api::GithubEvent;
use log::warn;
use serde::{Deserialize, Serialize};

use crate::actions::{
    closure_action::ClosureActionParams,
    github_action::GithubActionParams,
    merge_action::MergeActionParamsBuilder,
    MergeActionParams,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Actions {
    #[serde(rename = "merge")]
    AutoMerge(Box<MergeActionParams>),
    // An action that executes an arbitrary closure when the rule is triggered. Use with care. With great power comes
    // great responsibility.
    #[serde(rename = "closure")]
    Closure(Box<ClosureActionParams>),
    #[serde(rename = "github")]
    Github(Box<GithubActionParams>),
    // An action that does nothing. Generally constructed when a Rule is not well defined
    #[serde(rename = "none")]
    NullAction,
}

impl Actions {
    pub fn closure() -> ClosureActionBuilder {
        ClosureActionBuilder::new()
    }

    pub fn github() -> GithubActionBuilder {
        GithubActionBuilder::new()
    }

    pub fn auto_merge() -> MergeActionBuilder {
        MergeActionBuilder::default()
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

    pub fn with<F: Fn(String, Option<GithubEvent>) + Send + Sync + 'static>(mut self, f: F) -> Self {
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

    pub fn label_conflicts(mut self) -> Self {
        self.params = Some(GithubActionParams::check_conflicts());
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

pub struct MergeActionBuilder {
    params: MergeActionParamsBuilder,
}

impl Default for MergeActionBuilder {
    fn default() -> Self {
        Self {
            params: MergeActionParams::builder(),
        }
    }
}

impl MergeActionBuilder {
    pub fn with_min_acks(mut self, acks: usize) -> Self {
        self.params = self.params.acks_required(acks);
        self
    }

    pub fn with_min_reviews(mut self, reviews: usize) -> Self {
        self.params = self.params.reviews_required(reviews);
        self
    }

    pub fn skip_checks(mut self) -> Self {
        self.params = self.params.all_checks_must_pass(false);
        self
    }

    pub fn with_merge_label(mut self, label: &str) -> Self {
        self.params = self.params.merge_label(label);
        self
    }

    pub fn auto_merge(mut self) -> Self {
        self.params = self.params.perform_merge(true);
        self
    }

    pub fn add_ack_pattern(mut self, pattern: &str) -> Self {
        self.params = self.params.ack_pattern(pattern);
        self
    }

    pub fn build(self) -> Actions {
        let params = self.params.build();
        Actions::AutoMerge(Box::new(params))
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

    #[test]
    fn test_github_action_builder_conflicts() {
        let action = Actions::github().label_conflicts().build();

        match action {
            Actions::Github(p) => assert_eq!(*p, GithubActionParams::CheckConflicts),
            _ => panic!("Expected a CheckConflicts action"),
        }
    }

    #[test]
    fn auto_merge_builder() {
        let action = Actions::auto_merge()
            .with_min_acks(2)
            .with_min_reviews(1)
            .skip_checks()
            .with_merge_label("moo-rge")
            .auto_merge()
            .add_ack_pattern("whack")
            .build();

        match action {
            Actions::AutoMerge(p) => {
                assert_eq!(p.min_acks_required(), 2);
                assert_eq!(p.min_reviews_required(), 1);
                assert_eq!(p.all_checks_must_pass(), false);
                assert_eq!(p.merge_label(), "moo-rge");
                assert_eq!(p.perform_merge(), true);
                assert!(p.is_ack("whack"));
            },
            _ => panic!("Expected an AutoMerge action"),
        }
    }
}
