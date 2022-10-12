//! This module provides additional functionality and helper functions to the structs in the `models` module.
//! The code is kept separate to avoid messing with the code generation tools.

mod check_suite_event;
mod deserializers;
mod pull_request;
mod pull_request_event;
mod pull_request_review_comment_event;
mod pull_request_review_event;
pub use deserializers::*;
pub use pull_request::*;
