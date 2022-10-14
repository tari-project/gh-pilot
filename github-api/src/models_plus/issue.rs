use crate::models::Issue;

impl Issue {
    /// Is this issue _actually_ a pull request?
    pub fn is_pull_request(&self) -> bool {
        self.pull_request.is_some()
    }
}
