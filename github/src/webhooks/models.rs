use serde::{Deserialize, Serialize};
use crate::models::{Branch, Commit, CommitComment, Committer, InstallationLite, Issue, IssueComment, Label, Organization, PullRequest, PullRequestReview, PullRequestReviewComment, Repository, SimpleUser, Url};

//----------------------------------  Fields common to all events ------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonEventFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    organization: Option<Organization>,
    repository: Repository,
    sender: SimpleUser,
}

//----------------------------------      CommitComment Event     ------------------------------------------------------

/// A commit comment is created. The type of activity is specified in the `action` property.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitCommentEvent {
    /// The action performed. Can be `created`.
    pub action: CommitCommentAction,
    pub comment: CommitComment,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommitCommentAction {
    #[serde(rename = "created")]
    Created
}

//----------------------------------       IssueComment Event     ------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEvent {
    pub action: IssueCommentAction,
    pub comment: IssueComment,
    /// The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to.
    pub issue: Issue,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IssueCommentAction {
    #[doc = "issue_comment created event"]
    #[serde(rename = "created")]
    Created,
    #[doc = "issue_comment deleted event"]
    #[serde(rename = "deleted")]
    Deleted,
    #[doc = "issue_comment edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        comment: IssueComment,
    },
}

impl ToString for IssueCommentAction {
    fn to_string(&self) -> String {
        match *self {
            Self::Created => "created".to_string(),
            Self::Deleted => "deleted".to_string(),
            Self::Edited{..} => "edited".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

/// The changes to the comment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssueCommentEditedChangesBody>,
}

//----------------------------------         Issues Event        ------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEvent {
    action: IssuesEventAction,
    issue: Issue,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="lowercase")]
pub enum IssuesEventAction {
    /// issues assigned event. Activity related to an issue. The type of activity is specified in the action property.
    Assigned {
        /// The optional user who was assigned or unassigned from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<SimpleUser>,
    },
    /// issues closed event
    Closed,
    /// issues deleted event
    Deleted,
    /// issues demilestoned event
    Demilestoned,
    /// issues edited event
    Edited {
        changes: IssuesEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues labeled event
    Labeled {
        /// The optional label that was added or removed from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues locked event
    Locked,
    /// issues milestoned event
    Milestoned,
    /// issues opened event
    Opened,
    /// issues pinned event
    Pinned,
    /// issues reopened event
    Reopened,
    /// issues transferred event
    Transferred,
    /// issues unassigned event
    Unassigned {
        /// The optional user who was assigned or unassigned from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<SimpleUser>,
    },
    /// issues unlabeled event
    Unlabeled {
        /// The optional label that was added or removed from the issue.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
    },
    /// issues unlocked event
    Unlocked,
    /// issues unpinned event
    Unpinned,
}

impl ToString for IssuesEventAction {
    fn to_string(&self) -> String {
        match *self {
            Self::Assigned{..} => "assigned".to_string(),
            Self::Closed => "closed".to_string(),
            Self::Deleted => "deleted".to_string(),
            Self::Demilestoned => "demilestoned".to_string(),
            Self::Edited{..} => "edited".to_string(),
            Self::Labeled{..} => "labeled".to_string(),
            Self::Unlabeled{..} => "unlabeled".to_string(),
            Self::Locked => "locked".to_string(),
            Self::Milestoned => "milestoned".to_string(),
            Self::Unlocked => "unlocked".to_string(),
            Self::Opened => "opened".to_string(),
            Self::Pinned => "pinned".to_string(),
            Self::Unpinned => "unpinned".to_string(),
            Self::Reopened => "reopened".to_string(),
            Self::Unassigned{..} => "unassigned".to_string(),
            Self::Transferred => "transferred".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChangesTitle {
    /// The previous version of the title.
    pub from: String,
}

/// The changes to the issue.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssuesEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssuesEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<IssuesEditedChangesTitle>,
}
//------------------------------------        Label Event       --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEvent {
    pub action: LabelEventAction,
    /// The label that was added.
    pub label: Label,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LabelEventAction {
    /// label created event
    #[serde(rename = "created")]
    Created,
    /// label deleted event
    #[serde(rename = "deleted")]
    Deleted,
    /// label edited event
    #[serde(rename = "edited")]
    Edited(Box<LabelEditedChanges>),
}

/// The changes to the label if the action was `edited`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<LabelEditedChangesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LabelEditedChangesName>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChangesColor {
    /// The previous version of the color if the action was `edited`.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LabelEditedChangesName {
    /// The previous version of the name if the action was `edited`.
    pub from: String,
}

//------------------------------------         Ping Event       --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEvent {
    pub hook: PingEventHook,
    pub hook_id: i64,
    pub zen: String,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

/// The [webhook configuration](https://docs.github.com/en/rest/reference/repos#get-a-repository-webhook).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHook {
    pub active: bool,
    pub config: PingEventHookConfig,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_response: Option<PingEventHookLastResponse>,
    pub name: String,
    pub ping_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_url: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHookConfig {
    pub content_type: String,
    pub insecure_ssl: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingEventHookLastResponse {
    pub code: (),
    pub message: (),
    pub status: String,
}

//------------------------------------    PullRequest Event     --------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    /// The pull request number.
    pub number: u64,
    pub pull_request: PullRequest,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="snake_case")]
pub enum PullRequestAction {
    Assigned {
        assignee: SimpleUser,
    },
    AutoMergeDisabled,
    AutoMergeEnabled,
    /// If the action is closed and the merged key is false, the pull request was closed with unmerged commits. If
    /// the action is closed and the merged key is true, the pull request was merged.
    Closed,
    ConvertedToDraft,
    Edited {
        changes: PullRequestEditedChanges,
    },
    Labeled {
        label: Label,
    },
    Locked,
    Opened,
    ReadyForReview,
    Reopened,
    ReviewRequestRemoved {
        requested_reviewer: SimpleUser,
    },
    ReviewRequested {
        requested_reviewer: SimpleUser,
    },
    /// Triggered when a pull request's head branch is updated. For example, when the head branch is updated from the
    /// base branch, when new commits are pushed to the head branch, or when the base branch is changed.
    Synchronize {
        after: String,
        before: String,
    },
    Unassigned {
        assignee: SimpleUser,
    },
    Unlabeled {
        label: Label,
    },
    Unlocked,
}

/// The changes to the comment if the action was `edited`.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestEditedChangesBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<PullRequestEditedChangesTitle>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChangesBody {
    /// The previous version of the body if the action was `edited`.
    pub from: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestEditedChangesTitle {
    /// The previous version of the title if the action was `edited`.
    pub from: String,
}

//----------------------------------  Pull-Request-Review Event   ------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEvent {
    action: PullRequestReviewAction,
    review: PullRequestReview,
    pull_request: PullRequest,
    #[serde(flatten)]
    info: CommonEventFields,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PullRequestReviewAction {
    /// pull_request_review dismissed event
    #[serde(rename = "dismissed")]
    Dismissed,
    /// pull_request_review edited event
    #[serde(rename = "edited")]
    Edited {
        changes: PullRequestReviewEditedChanges,
    },
    /// pull_request_review submitted event
    #[serde(rename = "submitted")]
    Submitted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEditedChanges {
    pub body: PullRequestReviewEditedChangesBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewEditedChangesBody {
    /// The previous version of the body if the action was `edited`.
    pub from: String,
}

//----------------------------   Pull-Request-Review-Comment Event    --------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEvent {
    action: PullRequestReviewCommentAction,
    pull_request: PullRequest,
    comment: PullRequestReviewComment,
    #[serde(flatten)]
    info: CommonEventFields
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PullRequestReviewCommentAction {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "edited")]
    Edited(Box<PullRequestReviewCommentEditedChanges>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEditedChangesBody {
    /// The previous version of the body.
    pub from: String,
}

/// The changes to the comment.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequestReviewCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestReviewCommentEditedChangesBody>,
}

//---------------------------------------     Push event    ------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushEvent {
    /// The SHA of the most recent commit on `ref` after the push.
    pub after: String,
    pub base_ref: (),
    /// The SHA of the most recent commit on `ref` before the push.
    pub before: String,
    /// An array of commit objects describing the pushed commits.
    pub commits: Vec<Commit>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Option<Commit>,
    pub pusher: Committer,
    /// The full git ref that was pushed. Example: `refs/heads/main`.
    #[serde(rename = "ref")]
    pub git_ref: String,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

//---------------------------------------    Status event   ------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// An array of branch objects containing the status' SHA. Each branch contains the given SHA, but the SHA               may or may not be the head of the branch. The array includes a maximum of 10 branches.
    pub branches: Vec<Branch>,
    pub commit: Commit,
    pub context: String,
    pub created_at: String,
    /// The optional human-readable description added to the status.
    pub description: Option<String>,
    /// The unique identifier of the status.
    pub id: i64,
    pub name: String,
    /// The Commit SHA.
    pub sha: String,
    /// The new state. Can be `pending`, `success`, `failure`, or `error`.
    pub state: StatusEventState,
    /// The optional link added to the status.
    pub target_url: Option<Url>,
    pub updated_at: String,
    #[serde(flatten)]
    pub info: CommonEventFields,
}

/// The new state. Can be `pending`, `success`, `failure`, or `error`.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StatusEventState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "error")]
    Error,
}

impl ToString for StatusEventState {
    fn to_string(&self) -> String {
        match *self {
            StatusEventState::Pending => "pending".to_string(),
            StatusEventState::Success => "success".to_string(),
            StatusEventState::Failure => "failure".to_string(),
            StatusEventState::Error => "error".to_string(),
        }
    }
}
