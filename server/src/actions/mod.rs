//! This module implements the different Action classes supported by Github Pilot.
//!
//! Currently the following actions are supported:
//! - ClosureAction: executes an arbitrary closure (as long as it's Send + Sync)
//! - GithubAction: executes tasks on Github via the Github API
//!
//! To write a new Action implementation, you need to do the following
//!  - Define a new struct that implements [`actix::Actor`], `MyHotActionExecutor`, say.
//!  - Define a new struct, `MyHotActionParams` that completely defines what inputs `MyHotActionExecutor` needs to
//!    execute the task.
//!  - You'll also need a `MyHotActionMessage` struct that implements [`actix:Message`]. This struct must collect the
//!    data from `MyHotActionParams` (or even the entire struct) and combine it with the github event that's relevant to
//!    the executor. This usually will include the event struct itself.
//!  - Add any convenience constructors on `MyHotActionParams` to provide a beautiful API to rule writers.
//!  - Write a message handler for `MyHotActionMessage` in `MyHotActionExecutor`
//!  - Add `MyHotAction` as a field in the [`essentials::Actions`] enum using `MyHotActionParams` as the variant type.
//!  - Handle the new action type in [`PubSubActor::dispatch_message`].

mod closure_action;
mod essentials;
mod github_action;
mod merge_action;

pub use closure_action::{ClosureActionExecutor, ClosureActionMessage, ClosureActionParams};
pub use essentials::Actions;
pub use github_action::{GithubActionExecutor, GithubActionMessage, GithubActionParams};
pub use merge_action::{MergeActionMessage, MergeActionParams, MergeExecutor};
