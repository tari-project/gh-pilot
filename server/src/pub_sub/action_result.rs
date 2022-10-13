use std::error::Error;

pub enum ActionResult {
    Success,
    ConditionsNotMet,
    Failed,
    Indeterminate,
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
