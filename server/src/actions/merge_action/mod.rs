//! # The Github Pilot merge action
//!
//! An action for controlling automatic merges with Github Pilot.
//!
//! In general the merge flow goes as follows:
//! 1. The action is triggered by a webhook event, e.g. a PR, or label being added etc. This can be set up in
//! different ways, but if you require a manual review step at some point (recommended), then you may want to trigger
//! this action when a `P-approved` label or similar is added to a PR.
//! 2. The action can be configured to perform a number of checks:
//!   a. Check that the PR is mergeable, according to Github.
//!   b. Check that the required number of "ACK", or "+1" comments have been made by known contributors.
//!   c. Check that the required number of approving reviews have been made by maintainers.
//!   d. Check that the required "auto-merge" label is present (can be the same label that triggers this action).
//! 3. If all but 2a pass, you can _try_ to update and rebase the branch to make it mergeable using the
//!    [`AutoUpdate`] action.
//! 4. If all checks pass, the PR is merged using the merge strategy defined in the [`MergeAction`].

mod action_params;
mod executor;
mod message;

pub use action_params::MergeActionParams;
pub use executor::MergeExecutor;
pub use message::MergeActionMessage;
