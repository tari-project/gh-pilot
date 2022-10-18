use std::{error::Error, fmt::Display};

pub enum ActionResult {
    Success,
    ConditionsNotMet,
    Failed,
    Indeterminate,
}

impl Display for ActionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionResult::Success => write!(f, "Success"),
            ActionResult::ConditionsNotMet => write!(f, "ConditionsNotMet"),
            ActionResult::Failed => write!(f, "Failed"),
            ActionResult::Indeterminate => write!(f, "Indeterminate"),
        }
    }
}

impl ActionResult {
    pub fn from_result<T, E, F1, F2>(result: Result<T, E>, on_success: F1, on_failure: F2) -> Self
    where
        E: Error,
        F1: FnOnce(),
        F2: FnOnce(E),
    {
        match result {
            Ok(_) => {
                on_success();
                ActionResult::Success
            },
            Err(e) => {
                on_failure(e);
                ActionResult::Failed
            },
        }
    }
}
