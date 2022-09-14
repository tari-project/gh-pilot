//! This module implements the different Action classes supported by Github Pilot.
//!
//! Currently the following actions are supported:
//! - ClosureAction: executes an arbitrary closure (as long as it's Send + Sync)
//! - GithubAction: executes tasks on Github via the Github API
//!
//! To write a new Action implementation, you need to do the following
//!  - Define a new struct that implements [`actix::Actor`], `MyHotActionExecutor`, say.
//!  - Define a new struct, `MyHotAction` that completely defines what inputs `MyHotActionExecutor` needs to execute the
//!    task. By convention, the message structs are suffixed with `Action` since this is what is used in the Rule API.
//!  - Mark `MyHotAction` with the [`essentials::Action`] marker trait so that it can be used in [`crate::rules::Rule`]
//!    definitions.
//!  - You'll also need a `MyHotActionMessage` struct that implements [`actix:Message`]. This struct must collect the
//!    data from `MyHotAction` (or even the entire struct) and combine it with the github event that's relvant to the
//!    executor. This usually will include the event struct itself.
//!  - Add any convenience constructors on `MyHotAction` to provide a beautiful API to rule writers.
//!  - Write a message handler for `MyHotActionMessage` in `MyHotActionExecutor`
//!  - Add `MyHotActionExecutor` as a field in the `Executor` struct in PubSubActor.
//!  - Register a `MyHotActionExecutor` instance with the PubSub Actor in server.rs

mod closure_action;
mod essentials;
mod github_action;

pub use closure_action::{ClosureActionExecutor, ClosureActionMessage, ClosureActionParams};
pub use essentials::Actions;
pub use github_action::{GithubActionExecutor, GithubActionMessage, GithubActionParams};
