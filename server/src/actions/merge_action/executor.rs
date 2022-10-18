use std::sync::Arc;

use actix::{Actor, Addr, Context, Handler, ResponseFuture, Running, Supervised, SystemService};
use github_pilot_api::{
    error::GithubProviderError,
    graphql::{run_status::check_run_status_ql, CheckRunStatus, PullRequestComments},
    models_plus::{MergeMethod, MergeParameters},
    provider_traits::{
        CheckRunStatusProvider,
        Contributors,
        IssueProvider,
        PullRequestCommentsProvider,
        PullRequestProvider,
        PullRequestReviewSummary,
    },
    wrappers::IssueId,
    GithubEvent,
    GithubProvider,
};
use log::*;

use crate::{
    actions::merge_action::{message::MergeActionMessage, MergeActionParams},
    events::{BroadcastEvent, BroadcastEventMessage, Progress},
    pub_sub::{ActionResult, PubSubActor},
};

#[derive(Clone)]
pub struct MergeExecutor {
    provider: Arc<dyn PullRequestProvider>,
    comments: Arc<dyn PullRequestCommentsProvider>,
    reviews: Arc<dyn PullRequestReviewSummary>,
    contributors: Arc<dyn Contributors>,
    checks: Arc<dyn CheckRunStatusProvider>,
    issues: Arc<dyn IssueProvider>,
}

impl Default for MergeExecutor {
    fn default() -> Self {
        let provider = Arc::new(GithubProvider::default());
        MergeExecutor {
            provider: provider.clone(),
            comments: provider.clone(),
            reviews: provider.clone(),
            contributors: provider.clone(),
            checks: provider.clone(),
            issues: provider,
        }
    }
}

impl MergeExecutor {
    pub fn new(
        provider: Arc<dyn PullRequestProvider>,
        comments: Arc<dyn PullRequestCommentsProvider>,
        reviews: Arc<dyn PullRequestReviewSummary>,
        contributors: Arc<dyn Contributors>,
        checks: Arc<dyn CheckRunStatusProvider>,
        issues: Arc<dyn IssueProvider>,
    ) -> Self {
        MergeExecutor {
            provider,
            comments,
            reviews,
            contributors,
            checks,
            issues,
        }
    }

    /// Obtains a list of contributors from the [`Contributors`] provider.
    async fn fetch_contributors(&self, id: &IssueId) -> Result<Vec<String>, GithubProviderError> {
        let contributors = self
            .contributors
            .fetch_contributors(id.owner.as_str(), id.repo.as_str())
            .await?;
        Ok(contributors.into_iter().map(|c| c.login).collect())
    }

    /// Checks that the PR comments contain enough ACK comments from contributors
    async fn check_acks(
        &self,
        params: &MergeActionParams,
        id: &IssueId,
        contributors: Vec<String>,
        bcast: Option<Addr<PubSubActor>>,
        github_event: &GithubEvent,
    ) -> bool {
        let comments = match self.comments.fetch_pull_request_comments(id).await {
            Ok(comments) => comments,
            Err(e) => {
                warn!("⏫ Could not check ACK count because we could not get comments for PR {id}. {e}");
                return false;
            },
        };
        trace!("⏫ PR {id} has {} comments", comments.comments.len());
        let progress = Self::count_acks_sync(params, contributors, comments);
        debug!("⏫ PR {id} has {progress} required ACKs");
        let acks_done = progress.current >= params.min_acks_required();
        if acks_done {
            Self::broadcast(bcast, BroadcastEvent::AcksThresholdReached, github_event);
        } else {
            Self::broadcast(bcast, BroadcastEvent::AcksNeeded(Box::new(progress)), github_event);
        }
        acks_done
    }

    fn broadcast(bcast: Option<Addr<PubSubActor>>, event: BroadcastEvent, github_event: &GithubEvent) {
        if bcast.is_some() {
            let msg = BroadcastEventMessage::new(event, Some(github_event.clone()));
            PubSubActor::broadcast_event(&bcast, msg);
        }
    }

    // sync function to make it easier to test
    fn count_acks_sync(
        params: &MergeActionParams,
        mut contributors: Vec<String>,
        comments: PullRequestComments,
    ) -> Progress {
        // Ignoring review comment threads for ACKS
        let num_acks = comments
            .comments
            .iter()
            .filter(|&c| {
                if !params.is_ack(c.body.as_str()) {
                    return false;
                }
                if let Some(i) = contributors.iter().position(|u| u == &c.author) {
                    // A contributor can only ACK once
                    contributors.remove(i);
                    true
                } else {
                    false
                }
            })
            .count();
        Progress::new(num_acks, params.min_acks_required())
    }

    /// Checks whether the minimum number of reviews from maintainers have been submitted. If changes have been
    /// requested, this method always returns false.
    async fn check_reviews(
        &self,
        params: &MergeActionParams,
        id: &IssueId,
        bcast: Option<Addr<PubSubActor>>,
        github_event: &GithubEvent,
    ) -> bool {
        let reviews = match self.reviews.fetch_review_summary(id).await {
            Ok(reviews) => reviews,
            Err(e) => {
                warn!("👀 Could not check review count because we could not get reviews for PR {id}. {e}");
                return false;
            },
        };
        let approved = reviews.approvals();
        let change_req = reviews.changes_requested();
        let required = params.min_reviews_required();
        let total = reviews.total();
        debug!("👀 PR {id} has {total} reviews, {approved}/{required} required, changes_requested: {change_req}");
        let reviews_achieved = !change_req && approved >= required;
        if change_req {
            Self::broadcast(bcast.clone(), BroadcastEvent::ChangesRequested, github_event);
        }
        if reviews_achieved {
            Self::broadcast(bcast, BroadcastEvent::ReviewsThresholdReached, github_event);
        } else {
            let progress = Progress::new(approved, required);
            Self::broadcast(bcast, BroadcastEvent::ReviewsNeeded(Box::new(progress)), github_event);
        }
        reviews_achieved
    }

    /// Checks whether the branch protection checks have passed yet
    async fn checks_passed(&self, params: &MergeActionParams, id: &IssueId) -> bool {
        if !params.all_checks_must_pass() {
            debug!("⏫ MergeAction config does not require status checks to pass. Happy to proceed");
            return true;
        }
        let checks = match self.checks.fetch_check_run(id).await {
            Ok(checks) => checks,
            Err(e) => {
                warn!("⏫ Could not check check run status because we could not get checks for PR {id}. {e}");
                return false;
            },
        };
        trace!("⏫ Checking status of last Check Run for PR {id}");
        Self::have_all_required_checks_passed(&checks)
    }

    // Check that all _required_ checks have passed successfully
    fn have_all_required_checks_passed(checks: &CheckRunStatus) -> bool {
        debug!("⏫ The roll up status is {:?}", checks.overall_status());
        let (total, required, passed) = checks.checks().fold((0, 0, 0), |(total, required, passed), c| {
            let total = total + 1;
            let required = required + i32::from(c.is_required);
            let passed = passed + i32::from(matches!(c.result, check_run_status_ql::CheckConclusionState::SUCCESS));
            (total, required, passed)
        });
        debug!("⏫ PR has {total} checks, {passed} passed / {required} required.");
        // First look at the rollup status
        match checks.overall_status() {
            Some(check_run_status_ql::StatusState::SUCCESS) => {
                debug!("⏫ Github reports that the rolled up status of the PR checks is SUCCESS");
                true
            },
            Some(check_run_status_ql::StatusState::FAILURE) => {
                debug!("⏫ Github reports that the rolled up status of the PR checks is FAILURE");
                false
            },
            _ => passed >= required,
        }
    }

    /// Carries out the merge action. The action depends on the state of `[MergeActionParams::auto-merge]`.
    async fn execute_merge_action(&self, params: &MergeActionParams, id: &IssueId) -> ActionResult {
        debug!("⏫🟢 Executing merge action for PR {id}");
        let label = params.merge_label();
        let merge_label_status = self.check_merge_label(label, id).await;
        match (merge_label_status, params.perform_merge()) {
            (Ok(true), true) => self.merge_pr(id).await,
            (Ok(false), false) => self.add_label(id, params.merge_label()).await,
            (Ok(false), true) => {
                info!(
                    "⏫ We _could_ have merged this PR, but the merge label, `{label}` is not attached to the PR, so \
                     we thought it best not to right now."
                );
                ActionResult::ConditionsNotMet
            },
            (Err(e), true) => {
                warn!(
                    "⏫ We wanted to merged this PR, but could not check that the merge label, `{label}` was present \
                     because of some error: {e}. You should merge manually, or resolve the issue and trigger this \
                     action again."
                );
                ActionResult::Failed
            },
            (Err(e), false) => {
                warn!(
                    "⏫ We wanted to add the merge label, `{label}` but couldn't determine if it was already present, \
                     due to this error: {e}. You can add the label manually, or resolve the issue and trigger this \
                     action again."
                );
                ActionResult::Failed
            },
            (Ok(true), false) => {
                debug!("⏫ Was about to add merge label, `{label}`, but it's already added. Nothing more to do here.");
                ActionResult::Success
            },
        }
    }

    async fn check_merge_label(&self, label: &str, id: &IssueId) -> Result<bool, GithubProviderError> {
        self.issues.label_exists(label, id).await
    }

    async fn merge_pr(&self, id: &IssueId) -> ActionResult {
        let params = MergeParameters {
            merge_method: MergeMethod::Squash,
            ..Default::default()
        };
        debug!("⏫🟢 Attempting to merge PR {id}.");
        let res = self.provider.merge_pull_request(id, params).await;
        ActionResult::from_result(
            res,
            || info!("⏫ Merged PR {id}. Thank you for using AutoMerge™"),
            |e| warn!("⏫ Could not merge PR {id}. {e}"),
        )
    }

    async fn add_label(&self, id: &IssueId, label: &str) -> ActionResult {
        let res = self.issues.add_label(id, label).await;
        ActionResult::from_result(
            res,
            || info!("⏫ Added label {label} to PR {id}. Thank you for using AutoMerge™"),
            |e| warn!("⏫ Could not add label {label} to PR {id}. {e}"),
        )
    }
}

impl Actor for MergeExecutor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        debug!("⏫ MergeExecutor started");
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("⏫ MergeExecutor stopping");
        Running::Stop
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        debug!("⏫ MergeExecutor stopped");
    }
}

impl Supervised for MergeExecutor {}

impl SystemService for MergeExecutor {}

impl Handler<MergeActionMessage> for MergeExecutor {
    type Result = ResponseFuture<ActionResult>;

    fn handle(&mut self, msg: MergeActionMessage, _ctx: &mut Self::Context) -> Self::Result {
        let this = self.clone();
        let fut = async move {
            let MergeActionMessage {
                name,
                event_name,
                event,
                params,
                broadcaster,
            } = msg;
            debug!("⏫ MergeExecutor handler is running task \"{name}\" for event \"{event_name}\"");
            trace!("⏫ Event summary: {}", event.summary());
            let id = match event.related_pull_request() {
                Some(pr) => {
                    debug!("⏫ Extracted pull request details for {pr}");
                    pr
                },
                None => {
                    debug!("⏫ Could not find a related pull request for event {event_name}");
                    return ActionResult::Failed;
                },
            };
            let contributors = match this.fetch_contributors(&id).await {
                Ok(contributors) => {
                    trace!("⏫ Fetched {} contributors to {id}", contributors.len());
                    contributors
                },
                Err(e) => {
                    warn!("⏫ Merge action could not get the list of contributors for PR {id}. {e}");
                    return ActionResult::Failed;
                },
            };
            let acks_passed = this
                .check_acks(&params, &id, contributors, broadcaster.clone(), &event)
                .await;
            let reviews_passed = this.check_reviews(&params, &id, broadcaster, &event).await;
            let checks_passed = this.checks_passed(&params, &id).await;
            if acks_passed && reviews_passed && checks_passed {
                this.execute_merge_action(&params, &id).await
            } else {
                ActionResult::ConditionsNotMet
            }
        };
        Box::pin(fut)
    }
}

#[cfg(test)]
mod test {
    use async_trait::async_trait;
    use github_pilot_api::{
        error::GithubProviderError,
        graphql::PullRequestComments,
        models::Contributor,
        provider_traits::Contributors,
    };

    use crate::actions::merge_action::{MergeActionParams, MergeExecutor};

    pub struct MockProvider {
        contributors: Vec<String>,
    }

    impl MockProvider {
        pub fn new<S: Into<String>>(contributors: Vec<S>) -> Self {
            MockProvider {
                contributors: contributors.into_iter().map(|s| s.into()).collect(),
            }
        }
    }

    #[async_trait]
    impl Contributors for MockProvider {
        async fn fetch_contributors(&self, _owner: &str, _repo: &str) -> Result<Vec<Contributor>, GithubProviderError> {
            let contributors = self
                .contributors
                .iter()
                .map(|c| Contributor {
                    login: c.clone(),
                    ..Default::default()
                })
                .collect();
            Ok(contributors)
        }
    }

    #[tokio::test]
    async fn fetch_contributors() {
        let provider = MockProvider::new(vec!["foo", "bar"]);
        let contributors = provider.fetch_contributors("owner", "repo").await.unwrap();
        assert_eq!(contributors.len(), 2);
        assert_eq!(contributors[0].login, "foo");
        assert_eq!(contributors[1].login, "bar");
    }

    #[test]
    fn basic_acks() {
        let params = MergeActionParams::default();
        let mut comments = PullRequestComments::default();
        comments
            .add_comment("bob", "Hi")
            .add_comment("alice", "ACK")
            .add_comment("steve", "ACK")
            .add_comment("che", ":+1:")
            .add_comment("bob", "👍");
        let contributors = ["alice", "bob", "che"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(
            MergeExecutor::count_acks_sync(&params, contributors, comments).current,
            3
        );
    }

    #[test]
    fn sybil_attack() {
        let params = MergeActionParams::default();
        let mut comments = PullRequestComments::default();
        comments
            .add_comment("bob", "utACK")
            .add_comment("bob", "ACK")
            .add_comment("bob", "👍");
        let contributors = vec!["bob".into()];
        assert_eq!(
            MergeExecutor::count_acks_sync(&params, contributors, comments).current,
            1
        );
    }

    #[test]
    fn empty_vecs() {
        let params = MergeActionParams::default();
        let mut comments = PullRequestComments::default();
        comments.add_comment("bob", "ACK");
        let contributors = vec!["bob".into()];
        assert_eq!(MergeExecutor::count_acks_sync(&params, vec![], comments).current, 0);
        assert_eq!(
            MergeExecutor::count_acks_sync(&params, contributors, PullRequestComments::default()).current,
            0
        );
    }

    #[test]
    fn ambiguous_comments() {
        let params = MergeActionParams::default();
        let mut comments = PullRequestComments::default();
        comments
            .add_comment("alice", "Hi I'm BACK!")
            .add_comment("bob", "ACK")
            .add_comment("bob1", "ACK")
            .add_comment("rando", ":+1:")
            .add_comment("che", "Really good\n👍");
        let contributors = ["alice", "bob", "che"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        assert_eq!(
            MergeExecutor::count_acks_sync(&params, contributors, comments).current,
            2
        );
    }
}
