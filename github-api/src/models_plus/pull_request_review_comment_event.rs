use crate::{models::PullRequestReviewCommentEvent, wrappers::IssueId};

impl PullRequestReviewCommentEvent {
    pub fn related_pull_request(&self) -> IssueId {
        let owner = self.info.repository.owner.login.as_str();
        let repo = self.info.repository.name.as_str();
        let number = self.pull_request.number;
        IssueId::new(owner, repo, number)
    }
}
