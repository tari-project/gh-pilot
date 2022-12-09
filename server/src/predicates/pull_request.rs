use github_pilot_api::{
    models::{PullRequestAction, PullRequestEvent, PullRequestReviewEvent, ReviewState},
    newtype,
    GithubEvent,
};
use log::trace;
use serde::{Deserialize, Serialize};

use crate::{
    heuristics::pull_requests::{PullRequestComplexity, PullRequestHeuristics, PullRequestSize},
    pub_sub::GithubEventMessage,
    rules::RulePredicate,
};

newtype!(UserName, String, str);
newtype!(LabelName, String, str);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequest {
    Approved,
    Assigned(Option<UserName>),
    Labeled(Option<LabelName>),
    ReviewRequestRemoved(Option<UserName>),
    ReviewRequested(Option<UserName>),
    Unassigned(Option<UserName>),
    Unlabeled(Option<LabelName>),
    ClosedWithUnmergedCommits,
    Merged,
    ConvertedToDraft,
    Edited,
    Locked,
    Opened,
    ReadyForReview,
    Reopened,
    Synchronize,
    Unlocked,
    SizeGreaterThan(PullRequestSize),
    MoreComplexThan(PullRequestComplexity),
    PoorJustification,
}

impl PullRequest {
    pub fn closed_with_unmerged_commits() -> Self {
        Self::ClosedWithUnmergedCommits
    }

    pub fn merged() -> Self {
        Self::Merged
    }

    pub fn converted_to_draft() -> Self {
        Self::ConvertedToDraft
    }

    pub fn edited() -> Self {
        Self::Edited
    }

    pub fn locked() -> Self {
        Self::Locked
    }

    pub fn opened() -> Self {
        Self::Opened
    }

    pub fn ready_for_review() -> Self {
        Self::ReadyForReview
    }

    pub fn reopened() -> Self {
        Self::Reopened
    }

    pub fn synchronize() -> Self {
        Self::Synchronize
    }

    pub fn unlocked() -> Self {
        Self::Unlocked
    }

    pub fn assigned() -> Self {
        Self::Assigned(None)
    }

    pub fn assigned_to<S: Into<UserName>>(user_name: S) -> Self {
        Self::Assigned(Some(user_name.into()))
    }

    pub fn labeled() -> Self {
        Self::Labeled(None)
    }

    pub fn labeled_with<S: Into<LabelName>>(label: S) -> Self {
        Self::Labeled(Some(label.into()))
    }

    pub fn unlabeled() -> Self {
        Self::Unlabeled(None)
    }

    pub fn unlabeled_with<S: Into<LabelName>>(label: S) -> Self {
        Self::Unlabeled(Some(label.into()))
    }

    pub fn review_request_removed() -> Self {
        Self::ReviewRequestRemoved(None)
    }

    /// Predicate for checking when a review request for a requested reviewer has been removed
    pub fn review_request_removed_for<I: Into<UserName>>(user_name: I) -> Self {
        Self::ReviewRequestRemoved(Some(user_name.into()))
    }

    pub fn review_requested() -> Self {
        Self::ReviewRequested(None)
    }

    pub fn review_requested_for<I: Into<UserName>>(user_name: I) -> Self {
        Self::ReviewRequested(Some(user_name.into()))
    }

    pub fn unassigned() -> Self {
        Self::Unassigned(None)
    }

    pub fn unassigned_from<I: Into<UserName>>(user_name: I) -> Self {
        Self::Unassigned(Some(user_name.into()))
    }

    pub fn larger_than(size: PullRequestSize) -> Self {
        Self::SizeGreaterThan(size)
    }

    pub fn more_complex_than(complexity: PullRequestComplexity) -> Self {
        Self::MoreComplexThan(complexity)
    }

    pub fn poor_justification() -> Self {
        Self::PoorJustification
    }

    pub fn approved() -> Self {
        Self::Approved
    }
}

impl RulePredicate for PullRequest {
    fn matches(&self, event: &GithubEventMessage) -> bool {
        // The match expressions are quite complicated, so I've done multiple matches in the interest of readability
        // rather than one huge match.
        //
        // Firstly check if the event is a PullRequest event, which covers the majority of cases, and we will be
        // needing to interrogate the PR action field and the PR itself, so we extract those in the match
        if let GithubEvent::PullRequest(PullRequestEvent {
            action, pull_request, ..
        }) = event.event()
        {
            trace!("❓testing {self:?} against event {}/{action}", event.name());
            let heuristic = PullRequestHeuristics::new(pull_request);
            match (&self, action) {
                (PullRequest::Assigned(None), PullRequestAction::Assigned { .. }) => true,
                (PullRequest::Assigned(Some(user)), PullRequestAction::Assigned { assignee }) => {
                    assignee.name.is_some() && (user.as_ref() == assignee.name.as_ref().unwrap())
                },
                (PullRequest::Unassigned(None), PullRequestAction::Unassigned { .. }) => true,
                (PullRequest::Unassigned(Some(user)), PullRequestAction::Unassigned { assignee }) => {
                    assignee.name.is_some() && (user.as_ref() == assignee.name.as_ref().unwrap())
                },
                (PullRequest::Labeled(None), PullRequestAction::Labeled { .. }) => true,
                (PullRequest::Labeled(Some(label_wanted)), PullRequestAction::Labeled { label }) => {
                    label_wanted.as_ref() == label.name
                },
                (PullRequest::Unlabeled(None), PullRequestAction::Unlabeled { .. }) => true,
                (PullRequest::Unlabeled(Some(label_wanted)), PullRequestAction::Unlabeled { label }) => {
                    label_wanted.as_ref() == label.name
                },
                (PullRequest::ReviewRequestRemoved(None), PullRequestAction::ReviewRequestRemoved { .. }) => true,
                (
                    PullRequest::ReviewRequestRemoved(Some(user)),
                    PullRequestAction::ReviewRequestRemoved { requested_reviewer: r },
                ) => r.name.is_some() && (user.as_ref() == r.name.as_ref().unwrap()),
                (PullRequest::ReviewRequested(None), PullRequestAction::ReviewRequested { .. }) => true,
                (
                    PullRequest::ReviewRequested(Some(user)),
                    PullRequestAction::ReviewRequested { requested_reviewer: r },
                ) => r.name.is_some() && (user.as_ref() == r.name.as_ref().unwrap()),
                (PullRequest::ConvertedToDraft, PullRequestAction::ConvertedToDraft) => true,
                (PullRequest::Edited, PullRequestAction::Edited { .. }) => true,
                (PullRequest::Locked, PullRequestAction::Locked) => true,
                (PullRequest::Opened, PullRequestAction::Opened) => true,
                (PullRequest::ReadyForReview, PullRequestAction::ReadyForReview) => true,
                (PullRequest::Reopened, PullRequestAction::Reopened) => true,
                (PullRequest::Synchronize, PullRequestAction::Synchronize { .. }) => true,
                (PullRequest::Unlocked, PullRequestAction::Unlocked) => true,
                // If the action is closed and the merged key is false, the pull request was closed with unmerged
                // commits.
                (PullRequest::ClosedWithUnmergedCommits, PullRequestAction::Closed) => {
                    pull_request.merged == Some(false)
                },
                // If the action is closed and the merged key is true, the pull request was merged.
                (PullRequest::Merged, PullRequestAction::Closed) => pull_request.merged == Some(true),
                (
                    PullRequest::SizeGreaterThan(size),
                    PullRequestAction::Opened | PullRequestAction::Synchronize { .. } | PullRequestAction::Reopened,
                ) => heuristic.size() > *size,
                (
                    PullRequest::MoreComplexThan(complexity),
                    PullRequestAction::Opened | PullRequestAction::Edited { .. },
                ) => heuristic.complexity() > *complexity,
                (PullRequest::PoorJustification, PullRequestAction::Opened | PullRequestAction::Edited { .. }) => {
                    !heuristic.has_sufficient_context()
                },
                // Anything else does not match
                _ => false,
            }
        // For PR reviews, we must extract the pull request review event
        } else if let GithubEvent::PullRequestReview(PullRequestReviewEvent { review, .. }) = event.event() {
            match review.state {
                ReviewState::Approved => matches!(self, PullRequest::Approved),
                _ => false,
            }
        } else {
            // Anything else does not match
            false
        }
    }
}

#[cfg(test)]
mod test {
    use github_pilot_api::GithubEvent;

    use super::{LabelName, PullRequest, UserName};
    use crate::{pub_sub::GithubEventMessage, rules::RulePredicate};

    #[test]
    fn pr_predicate_simple() {
        let pred = PullRequest::merged();
        assert!(matches!(pred, PullRequest::Merged));
    }

    #[test]
    fn pr_predicates_with_usernames() {
        let pred = PullRequest::assigned();
        assert_eq!(pred, PullRequest::Assigned(None));
        let pred = PullRequest::assigned_to("Bob");
        let bob = UserName::from("Bob");
        assert_eq!(pred, PullRequest::Assigned(Some(bob.clone())));
        let pred = PullRequest::assigned_to("Bob".to_string());
        assert_eq!(pred, PullRequest::Assigned(Some(bob.clone())));

        let pred = PullRequest::review_request_removed();
        assert_eq!(pred, PullRequest::ReviewRequestRemoved(None));
        let pred = PullRequest::review_request_removed_for(bob.clone());
        assert_eq!(pred, PullRequest::ReviewRequestRemoved(Some(bob)));

        let pred = PullRequest::review_requested();
        assert_eq!(pred, PullRequest::ReviewRequested(None));
        let pred = PullRequest::unassigned();
        assert_eq!(pred, PullRequest::Unassigned(None));
    }

    #[test]
    fn pr_predicates_with_labels() {
        let pred = PullRequest::labeled();
        assert_eq!(pred, PullRequest::Labeled(None));
        let pred = PullRequest::labeled_with("bug");
        assert_eq!(pred, PullRequest::Labeled(Some(LabelName::from("bug"))));
        let pred = PullRequest::unlabeled_with("bug".to_string());
        assert_eq!(pred, PullRequest::Unlabeled(Some(LabelName::from("bug"))));
        let pred = PullRequest::unlabeled();
        assert_eq!(pred, PullRequest::Unlabeled(None));
    }

    #[test]
    fn sanity_checks() {
        let pred = PullRequest::closed_with_unmerged_commits();
        assert_eq!(pred, PullRequest::ClosedWithUnmergedCommits);
        let pred = PullRequest::merged();
        assert_eq!(pred, PullRequest::Merged);
        let pred = PullRequest::converted_to_draft();
        assert_eq!(pred, PullRequest::ConvertedToDraft);
        let pred = PullRequest::edited();
        assert_eq!(pred, PullRequest::Edited);
        let pred = PullRequest::locked();
        assert_eq!(pred, PullRequest::Locked);
        let pred = PullRequest::opened();
        assert_eq!(pred, PullRequest::Opened);
        let pred = PullRequest::ready_for_review();
        assert_eq!(pred, PullRequest::ReadyForReview);
        let pred = PullRequest::reopened();
        assert_eq!(pred, PullRequest::Reopened);
        let pred = PullRequest::synchronize();
        assert_eq!(pred, PullRequest::Synchronize);
        let pred = PullRequest::unlocked();
        assert_eq!(pred, PullRequest::Unlocked);
    }

    #[test]
    fn pull_request_predicate_matches() {
        let data = include_str!("../../test-data/pr_event.json");
        let event = GithubEvent::try_from_webhook_info("pull_request", data).unwrap();
        let msg = GithubEventMessage::new("test", event);
        assert!(PullRequest::opened().matches(&msg));
        assert_eq!(PullRequest::merged().matches(&msg), false);
    }

    #[test]
    fn pr_review_approval_matches() {
        let data = include_str!("../../test-data/pr_review_approved.json");
        let event = GithubEvent::try_from_webhook_info("pull_request_review", data).unwrap();
        let msg = GithubEventMessage::new("test", event);
        assert!(PullRequest::approved().matches(&msg));
    }
}
