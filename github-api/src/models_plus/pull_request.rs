use crate::models::PullRequest;

impl PullRequest {
    pub fn has_merge_conflicts(&self) -> bool {
        matches!(self.mergeable, Some(false)) && !matches!(self.merged, Some(true))
    }
}
