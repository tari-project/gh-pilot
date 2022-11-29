use github_pilot_api::{
    models::{PullRequestAction, PullRequestEvent, PullRequestReviewEvent, ReviewState},
    newtype,
    GithubEvent,
};
use log::trace;

use crate::{
    heuristics::pull_requests::{PullRequestComplexity, PullRequestHeuristics, PullRequestSize},
    pub_sub::GithubEventMessage,
    rules::RulePredicate,
};

newtype!(UserName, String, str);
newtype!(LabelName, String, str);

#[derive(Clone, Debug, PartialEq, Eq)]
enum PullRequestPredicate {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PullRequest {
    trigger: PullRequestPredicate,
}

impl PullRequest {
    pub fn closed_with_unmerged_commits() -> Self {
        Self {
            trigger: PullRequestPredicate::ClosedWithUnmergedCommits,
        }
    }

    pub fn merged() -> Self {
        Self {
            trigger: PullRequestPredicate::Merged,
        }
    }

    pub fn converted_to_draft() -> Self {
        Self {
            trigger: PullRequestPredicate::ConvertedToDraft,
        }
    }

    pub fn edited() -> Self {
        Self {
            trigger: PullRequestPredicate::Edited,
        }
    }

    pub fn locked() -> Self {
        Self {
            trigger: PullRequestPredicate::Locked,
        }
    }

    pub fn opened() -> Self {
        Self {
            trigger: PullRequestPredicate::Opened,
        }
    }

    pub fn ready_for_review() -> Self {
        Self {
            trigger: PullRequestPredicate::ReadyForReview,
        }
    }

    pub fn reopened() -> Self {
        Self {
            trigger: PullRequestPredicate::Reopened,
        }
    }

    pub fn synchronize() -> Self {
        Self {
            trigger: PullRequestPredicate::Synchronize,
        }
    }

    pub fn unlocked() -> Self {
        Self {
            trigger: PullRequestPredicate::Unlocked,
        }
    }

    pub fn assigned() -> Self {
        Self {
            trigger: PullRequestPredicate::Assigned(None),
        }
    }

    pub fn assigned_to<S: Into<UserName>>(user_name: S) -> Self {
        Self {
            trigger: PullRequestPredicate::Assigned(Some(user_name.into())),
        }
    }

    pub fn labeled() -> Self {
        Self {
            trigger: PullRequestPredicate::Labeled(None),
        }
    }

    pub fn labeled_with<S: Into<LabelName>>(label: S) -> Self {
        Self {
            trigger: PullRequestPredicate::Labeled(Some(label.into())),
        }
    }

    pub fn unlabeled() -> Self {
        Self {
            trigger: PullRequestPredicate::Unlabeled(None),
        }
    }

    pub fn unlabeled_with<S: Into<LabelName>>(label: S) -> Self {
        Self {
            trigger: PullRequestPredicate::Unlabeled(Some(label.into())),
        }
    }

    pub fn review_request_removed() -> Self {
        Self {
            trigger: PullRequestPredicate::ReviewRequestRemoved(None),
        }
    }

    /// Predicate for checking when a review request for a requested reviewer has been removed
    pub fn review_request_removed_for<I: Into<UserName>>(user_name: I) -> Self {
        Self {
            trigger: PullRequestPredicate::ReviewRequestRemoved(Some(user_name.into())),
        }
    }

    pub fn review_requested() -> Self {
        Self {
            trigger: PullRequestPredicate::ReviewRequested(None),
        }
    }

    pub fn review_requested_for<I: Into<UserName>>(user_name: I) -> Self {
        Self {
            trigger: PullRequestPredicate::ReviewRequested(Some(user_name.into())),
        }
    }

    pub fn unassigned() -> Self {
        Self {
            trigger: PullRequestPredicate::Unassigned(None),
        }
    }

    pub fn unassigned_from<I: Into<UserName>>(user_name: I) -> Self {
        Self {
            trigger: PullRequestPredicate::Unassigned(Some(user_name.into())),
        }
    }

    pub fn larger_than(size: PullRequestSize) -> Self {
        Self {
            trigger: PullRequestPredicate::SizeGreaterThan(size),
        }
    }

    pub fn more_complex_than(complexity: PullRequestComplexity) -> Self {
        Self {
            trigger: PullRequestPredicate::MoreComplexThan(complexity),
        }
    }

    pub fn poor_justification() -> Self {
        Self {
            trigger: PullRequestPredicate::PoorJustification,
        }
    }

    pub fn approved() -> Self {
        Self {
            trigger: PullRequestPredicate::Approved,
        }
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
            trace!("❓testing {:?} against event {}/{}", self.trigger, event.name(), action);
            let heuristic = PullRequestHeuristics::new(pull_request);
            match (&self.trigger, action) {
                (PullRequestPredicate::Assigned(None), PullRequestAction::Assigned { .. }) => true,
                (PullRequestPredicate::Assigned(Some(user)), PullRequestAction::Assigned { assignee }) => {
                    assignee.name.is_some() && (user.as_ref() == assignee.name.as_ref().unwrap())
                },
                (PullRequestPredicate::Unassigned(None), PullRequestAction::Unassigned { .. }) => true,
                (PullRequestPredicate::Unassigned(Some(user)), PullRequestAction::Unassigned { assignee }) => {
                    assignee.name.is_some() && (user.as_ref() == assignee.name.as_ref().unwrap())
                },
                (PullRequestPredicate::Labeled(None), PullRequestAction::Labeled { .. }) => true,
                (PullRequestPredicate::Labeled(Some(label_wanted)), PullRequestAction::Labeled { label }) => {
                    label_wanted.as_ref() == label.name
                },
                (PullRequestPredicate::Unlabeled(None), PullRequestAction::Unlabeled { .. }) => true,
                (PullRequestPredicate::Unlabeled(Some(label_wanted)), PullRequestAction::Unlabeled { label }) => {
                    label_wanted.as_ref() == label.name
                },
                (PullRequestPredicate::ReviewRequestRemoved(None), PullRequestAction::ReviewRequestRemoved { .. }) => {
                    true
                },
                (
                    PullRequestPredicate::ReviewRequestRemoved(Some(user)),
                    PullRequestAction::ReviewRequestRemoved { requested_reviewer: r },
                ) => r.name.is_some() && (user.as_ref() == r.name.as_ref().unwrap()),
                (PullRequestPredicate::ReviewRequested(None), PullRequestAction::ReviewRequested { .. }) => true,
                (
                    PullRequestPredicate::ReviewRequested(Some(user)),
                    PullRequestAction::ReviewRequested { requested_reviewer: r },
                ) => r.name.is_some() && (user.as_ref() == r.name.as_ref().unwrap()),
                (PullRequestPredicate::ConvertedToDraft, PullRequestAction::ConvertedToDraft) => true,
                (PullRequestPredicate::Edited, PullRequestAction::Edited { .. }) => true,
                (PullRequestPredicate::Locked, PullRequestAction::Locked) => true,
                (PullRequestPredicate::Opened, PullRequestAction::Opened) => true,
                (PullRequestPredicate::ReadyForReview, PullRequestAction::ReadyForReview) => true,
                (PullRequestPredicate::Reopened, PullRequestAction::Reopened) => true,
                (PullRequestPredicate::Synchronize, PullRequestAction::Synchronize { .. }) => true,
                (PullRequestPredicate::Unlocked, PullRequestAction::Unlocked) => true,
                // If the action is closed and the merged key is false, the pull request was closed with unmerged
                // commits.
                (PullRequestPredicate::ClosedWithUnmergedCommits, PullRequestAction::Closed) => {
                    pull_request.merged == Some(false)
                },
                // If the action is closed and the merged key is true, the pull request was merged.
                (PullRequestPredicate::Merged, PullRequestAction::Closed) => pull_request.merged == Some(true),
                (
                    PullRequestPredicate::SizeGreaterThan(size),
                    PullRequestAction::Opened | PullRequestAction::Synchronize { .. } | PullRequestAction::Reopened,
                ) => heuristic.size() > *size,
                (
                    PullRequestPredicate::MoreComplexThan(complexity),
                    PullRequestAction::Opened | PullRequestAction::Edited { .. },
                ) => heuristic.complexity() > *complexity,
                (
                    PullRequestPredicate::PoorJustification,
                    PullRequestAction::Opened | PullRequestAction::Edited { .. },
                ) => !heuristic.has_sufficient_context(),
                // Anything else does not match
                _ => false,
            }
        // For PR reviews, we must extract the pull request review event
        } else if let GithubEvent::PullRequestReview(PullRequestReviewEvent { review, .. }) = event.event() {
            match review.state {
                ReviewState::Approved => self.trigger == PullRequestPredicate::Approved,
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

    use super::{LabelName, PullRequest, PullRequestPredicate, UserName};
    use crate::{pub_sub::GithubEventMessage, rules::RulePredicate};

    #[test]
    fn pr_predicate_simple() {
        let pred = PullRequest::merged();
        assert!(matches!(pred.trigger, PullRequestPredicate::Merged));
    }

    #[test]
    fn pr_predicates_with_usernames() {
        let pred = PullRequest::assigned();
        assert_eq!(pred.trigger, PullRequestPredicate::Assigned(None));
        let pred = PullRequest::assigned_to("Bob");
        let bob = UserName::from("Bob");
        assert_eq!(pred.trigger, PullRequestPredicate::Assigned(Some(bob.clone())));
        let pred = PullRequest::assigned_to("Bob".to_string());
        assert_eq!(pred.trigger, PullRequestPredicate::Assigned(Some(bob.clone())));

        let pred = PullRequest::review_request_removed();
        assert_eq!(pred.trigger, PullRequestPredicate::ReviewRequestRemoved(None));
        let pred = PullRequest::review_request_removed_for(bob.clone());
        assert_eq!(pred.trigger, PullRequestPredicate::ReviewRequestRemoved(Some(bob)));

        let pred = PullRequest::review_requested();
        assert_eq!(pred.trigger, PullRequestPredicate::ReviewRequested(None));
        let pred = PullRequest::unassigned();
        assert_eq!(pred.trigger, PullRequestPredicate::Unassigned(None));
    }

    #[test]
    fn pr_predicates_with_labels() {
        let pred = PullRequest::labeled();
        assert_eq!(pred.trigger, PullRequestPredicate::Labeled(None));
        let pred = PullRequest::labeled_with("bug");
        assert_eq!(
            pred.trigger,
            PullRequestPredicate::Labeled(Some(LabelName::from("bug")))
        );
        let pred = PullRequest::unlabeled_with("bug".to_string());
        assert_eq!(
            pred.trigger,
            PullRequestPredicate::Unlabeled(Some(LabelName::from("bug")))
        );
        let pred = PullRequest::unlabeled();
        assert_eq!(pred.trigger, PullRequestPredicate::Unlabeled(None));
    }

    #[test]
    fn sanity_checks() {
        let pred = PullRequest::closed_with_unmerged_commits();
        assert_eq!(pred.trigger, PullRequestPredicate::ClosedWithUnmergedCommits);
        let pred = PullRequest::merged();
        assert_eq!(pred.trigger, PullRequestPredicate::Merged);
        let pred = PullRequest::converted_to_draft();
        assert_eq!(pred.trigger, PullRequestPredicate::ConvertedToDraft);
        let pred = PullRequest::edited();
        assert_eq!(pred.trigger, PullRequestPredicate::Edited);
        let pred = PullRequest::locked();
        assert_eq!(pred.trigger, PullRequestPredicate::Locked);
        let pred = PullRequest::opened();
        assert_eq!(pred.trigger, PullRequestPredicate::Opened);
        let pred = PullRequest::ready_for_review();
        assert_eq!(pred.trigger, PullRequestPredicate::ReadyForReview);
        let pred = PullRequest::reopened();
        assert_eq!(pred.trigger, PullRequestPredicate::Reopened);
        let pred = PullRequest::synchronize();
        assert_eq!(pred.trigger, PullRequestPredicate::Synchronize);
        let pred = PullRequest::unlocked();
        assert_eq!(pred.trigger, PullRequestPredicate::Unlocked);
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
