use serde::{Deserialize, Serialize};
mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AlertInstance {
    #[doc = "Identifies the configuration under which the analysis was executed. For example, in GitHub Actions this includes the workflow filename and job name."]
    pub analysis_key: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classifications: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_sha: Option<String>,
    #[doc = "Identifies the variable values associated with the environment in which the analysis that generated this alert instance was performed, such as the language that was analyzed."]
    pub environment: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<AlertInstanceLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<AlertInstanceMessage>,
    #[doc = "The full Git reference, formatted as `refs/heads/<branch name>`."]
    #[serde(rename = "ref")]
    pub ref_: String,
    #[doc = "State of a code scanning alert."]
    pub state: AlertInstanceState,
}
#[doc = "GitHub apps are a new way to extend GitHub. They can be installed directly on organizations and user accounts and granted access to specific repositories. They come with granular permissions and built-in webhooks. GitHub apps are first class actors within GitHub."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct App {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: Option<String>,
    #[doc = "The list of events for the GitHub app"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    pub external_url: String,
    pub html_url: String,
    #[doc = "Unique identifier of the GitHub app"]
    pub id: i64,
    #[doc = "The name of the GitHub app"]
    pub name: String,
    pub node_id: String,
    pub owner: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AppPermissions>,
    #[doc = "The slug name of the GitHub app"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
}
#[doc = "How the author is associated with the repository."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AuthorAssociation {
    #[serde(rename = "COLLABORATOR")]
    Collaborator,
    #[serde(rename = "CONTRIBUTOR")]
    Contributor,
    #[serde(rename = "FIRST_TIMER")]
    FirstTimer,
    #[serde(rename = "FIRST_TIME_CONTRIBUTOR")]
    FirstTimeContributor,
    #[serde(rename = "MANNEQUIN")]
    Mannequin,
    #[serde(rename = "MEMBER")]
    Member,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "OWNER")]
    Owner,
}
impl ToString for AuthorAssociation {
    fn to_string(&self) -> String {
        match *self {
            AuthorAssociation::Collaborator => "COLLABORATOR".to_string(),
            AuthorAssociation::Contributor => "CONTRIBUTOR".to_string(),
            AuthorAssociation::FirstTimer => "FIRST_TIMER".to_string(),
            AuthorAssociation::FirstTimeContributor => "FIRST_TIME_CONTRIBUTOR".to_string(),
            AuthorAssociation::Mannequin => "MANNEQUIN".to_string(),
            AuthorAssociation::Member => "MEMBER".to_string(),
            AuthorAssociation::None => "NONE".to_string(),
            AuthorAssociation::Owner => "OWNER".to_string(),
        }
    }
}
#[doc = "A deployment to a repository environment. This will only be populated if the check run was created by a GitHub Actions workflow job that references an environment."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunDeployment {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: Option<String>,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    pub original_environment: String,
    pub repository_url: String,
    pub statuses_url: String,
    pub task: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunPullRequest {
    pub base: CheckRunPullRequestBase,
    pub head: CheckRunPullRequestHead,
    pub id: i64,
    pub number: i64,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCompleted {
    pub action: CheckRunCompletedAction,
    pub check_run: CheckRunCompletedCheckRun,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    #[doc = "The action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_action: Option<CheckRunCompletedRequestedAction>,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCreated {
    pub action: CheckRunCreatedAction,
    pub check_run: CheckRunCreatedCheckRun,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    #[doc = "The action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_action: Option<CheckRunCreatedRequestedAction>,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRequestedAction {
    pub action: CheckRunRequestedActionAction,
    pub check_run: CheckRunRequestedActionCheckRun,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub requested_action: CheckRunRequestedActionRequestedAction,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRerequested {
    pub action: CheckRunRerequestedAction,
    pub check_run: CheckRunRerequestedCheckRun,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    #[doc = "The action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_action: Option<CheckRunRerequestedRequestedAction>,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum CheckRunEvent {
    #[doc = "check_run completed event"]
    #[serde(rename = "completed")]
    Completed {
        check_run: CheckRunCompletedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        #[doc = "The action requested by the user."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requested_action: Option<CheckRunCompletedRequestedAction>,
        sender: User,
    },
    #[doc = "check_run created event"]
    #[serde(rename = "created")]
    Created {
        check_run: CheckRunCreatedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        #[doc = "The action requested by the user."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requested_action: Option<CheckRunCreatedRequestedAction>,
        sender: User,
    },
    #[doc = "check_run requested_action event"]
    #[serde(rename = "requested_action")]
    RequestedAction {
        check_run: CheckRunRequestedActionCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        requested_action: CheckRunRequestedActionRequestedAction,
        sender: User,
    },
    #[doc = "check_run rerequested event"]
    #[serde(rename = "rerequested")]
    Rerequested {
        check_run: CheckRunRerequestedCheckRun,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        #[doc = "The action requested by the user."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requested_action: Option<CheckRunRerequestedRequestedAction>,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteCompleted {
    pub action: CheckSuiteCompletedAction,
    pub check_suite: CheckSuiteCompletedCheckSuite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteRequested {
    pub action: CheckSuiteRequestedAction,
    pub check_suite: CheckSuiteRequestedCheckSuite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteRerequested {
    pub action: CheckSuiteRerequestedAction,
    pub check_suite: CheckSuiteRerequestedCheckSuite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum CheckSuiteEvent {
    #[doc = "check_suite completed event"]
    #[serde(rename = "completed")]
    Completed {
        check_suite: CheckSuiteCompletedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "check_suite requested event"]
    #[serde(rename = "requested")]
    Requested {
        check_suite: CheckSuiteRequestedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "check_suite rerequested event"]
    #[serde(rename = "rerequested")]
    Rerequested {
        check_suite: CheckSuiteRerequestedCheckSuite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertAppearedInBranch {
    pub action: CodeScanningAlertAppearedInBranchAction,
    pub alert: CodeScanningAlertAppearedInBranchAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: GithubOrg,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertClosedByUser {
    pub action: CodeScanningAlertClosedByUserAction,
    pub alert: CodeScanningAlertClosedByUserAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertCreated {
    pub action: CodeScanningAlertCreatedAction,
    pub alert: CodeScanningAlertCreatedAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: GithubOrg,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertFixed {
    pub action: CodeScanningAlertFixedAction,
    pub alert: CodeScanningAlertFixedAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: GithubOrg,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopened {
    pub action: CodeScanningAlertReopenedAction,
    pub alert: CodeScanningAlertReopenedAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: GithubOrg,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedByUser {
    pub action: CodeScanningAlertReopenedByUserAction,
    pub alert: CodeScanningAlertReopenedByUserAlert,
    #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    pub commit_oid: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum CodeScanningAlertEvent {
    #[doc = "code_scanning_alert appeared_in_branch event"]
    #[serde(rename = "appeared_in_branch")]
    AppearedInBranch {
        alert: CodeScanningAlertAppearedInBranchAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: GithubOrg,
    },
    #[doc = "code_scanning_alert closed_by_user event"]
    #[serde(rename = "closed_by_user")]
    ClosedByUser {
        alert: CodeScanningAlertClosedByUserAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: User,
    },
    #[doc = "code_scanning_alert created event"]
    #[serde(rename = "created")]
    Created {
        alert: CodeScanningAlertCreatedAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: GithubOrg,
    },
    #[doc = "code_scanning_alert fixed event"]
    #[serde(rename = "fixed")]
    Fixed {
        alert: CodeScanningAlertFixedAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: GithubOrg,
    },
    #[doc = "code_scanning_alert reopened event"]
    #[serde(rename = "reopened")]
    Reopened {
        alert: CodeScanningAlertReopenedAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: GithubOrg,
    },
    #[doc = "code_scanning_alert reopened_by_user event"]
    #[serde(rename = "reopened_by_user")]
    ReopenedByUser {
        alert: CodeScanningAlertReopenedByUserAlert,
        #[doc = "The commit SHA of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        commit_oid: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        #[doc = "The Git reference of the code scanning alert. When the action is `reopened_by_user` or `closed_by_user`, the event was triggered by the `sender` and this value will be empty."]
        #[serde(rename = "ref")]
        ref_: String,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Commit {
    #[doc = "An array of files added in the commit."]
    pub added: Vec<String>,
    pub author: Committer,
    pub committer: Committer,
    #[doc = "Whether this commit is distinct from any that have been pushed before."]
    pub distinct: bool,
    pub id: String,
    #[doc = "The commit message."]
    pub message: String,
    #[doc = "An array of files removed in the commit."]
    pub modified: Vec<String>,
    #[doc = "An array of files modified by the commit."]
    pub removed: Vec<String>,
    #[doc = "The ISO 8601 timestamp of the commit."]
    pub timestamp: String,
    pub tree_id: String,
    #[doc = "URL that points to the commit API resource."]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CommitSimple {
    pub author: Committer,
    pub committer: Committer,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
}
#[doc = "A commit comment is created. The type of activity is specified in the `action` property. "]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CommitCommentCreated {
    #[doc = "The action performed. Can be `created`."]
    pub action: CommitCommentCreatedAction,
    pub comment: CommitCommentCreatedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitCommentEvent(pub CommitCommentCreated);
impl std::ops::Deref for CommitCommentEvent {
    type Target = CommitCommentCreated;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Metaproperties for Git author/committer information."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Committer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The git author's email address."]
    pub email: String,
    #[doc = "The git author's name."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ContentReferenceCreated {
    pub action: ContentReferenceCreatedAction,
    pub content_reference: ContentReferenceCreatedContentReference,
    pub installation: InstallationLite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentReferenceEvent(pub ContentReferenceCreated);
impl std::ops::Deref for ContentReferenceEvent {
    type Target = ContentReferenceCreated;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A Git branch or tag is created."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CreateEvent {
    #[doc = "The repository's current description."]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The name of the repository's default branch (usually `main`)."]
    pub master_branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The pusher type for the event. Can be either `user` or a deploy key."]
    pub pusher_type: String,
    #[doc = "The [`git ref`](https://docs.github.com/en/rest/reference/git#get-a-reference) resource."]
    #[serde(rename = "ref")]
    pub ref_: String,
    #[doc = "The type of Git ref object created in the repository. Can be either `branch` or `tag`."]
    pub ref_type: CreateEventRefType,
    pub repository: Repository,
    pub sender: User,
}
#[doc = "A Git branch or tag is deleted."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeleteEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The pusher type for the event. Can be either `user` or a deploy key."]
    pub pusher_type: String,
    #[doc = "The [`git ref`](https://docs.github.com/en/rest/reference/git#get-a-reference) resource."]
    #[serde(rename = "ref")]
    pub ref_: String,
    #[doc = "The type of Git ref object deleted in the repository. Can be either `branch` or `tag`."]
    pub ref_type: DeleteEventRefType,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeployKeyCreated {
    pub action: DeployKeyCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub key: DeployKeyCreatedKey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeployKeyDeleted {
    pub action: DeployKeyDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub key: DeployKeyDeletedKey,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum DeployKeyEvent {
    #[doc = "deploy_key created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        key: DeployKeyCreatedKey,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "deploy_key deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        key: DeployKeyDeletedKey,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentCreated {
    pub action: DeploymentCreatedAction,
    pub deployment: DeploymentCreatedDeployment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentEvent(pub DeploymentCreated);
impl std::ops::Deref for DeploymentEvent {
    type Target = DeploymentCreated;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentStatusCreated {
    pub action: DeploymentStatusCreatedAction,
    pub deployment: DeploymentStatusCreatedDeployment,
    pub deployment_status: DeploymentStatusCreatedDeploymentStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentStatusEvent(pub DeploymentStatusCreated);
impl std::ops::Deref for DeploymentStatusEvent {
    type Target = DeploymentStatusCreated;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Discussion {
    pub active_lock_reason: Option<String>,
    pub answer_chosen_at: Option<String>,
    pub answer_chosen_by: Option<User>,
    pub answer_html_url: Option<String>,
    pub author_association: AuthorAssociation,
    pub body: String,
    pub category: DiscussionCategory,
    pub comments: i64,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub html_url: String,
    pub id: i64,
    pub locked: bool,
    pub node_id: String,
    pub number: i64,
    pub repository_url: String,
    pub state: DiscussionState,
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionAnswered {
    pub action: DiscussionAnsweredAction,
    pub answer: DiscussionAnsweredAnswer,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCategoryChanged {
    pub action: DiscussionCategoryChangedAction,
    pub changes: DiscussionCategoryChangedChanges,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCreated {
    pub action: DiscussionCreatedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionDeleted {
    pub action: DiscussionDeletedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionEdited {
    pub action: DiscussionEditedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<DiscussionEditedChanges>,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionLocked {
    pub action: DiscussionLockedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionPinned {
    pub action: DiscussionPinnedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionTransferred {
    pub action: DiscussionTransferredAction,
    pub changes: DiscussionTransferredChanges,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionUnanswered {
    pub action: DiscussionUnansweredAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub old_answer: DiscussionUnansweredOldAnswer,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionUnlocked {
    pub action: DiscussionUnlockedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionUnpinned {
    pub action: DiscussionUnpinnedAction,
    pub discussion: Discussion,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentCreated {
    pub action: DiscussionCommentCreatedAction,
    pub comment: DiscussionCommentCreatedComment,
    pub discussion: Discussion,
    pub installation: InstallationLite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentDeleted {
    pub action: DiscussionCommentDeletedAction,
    pub comment: DiscussionCommentDeletedComment,
    pub discussion: Discussion,
    pub installation: InstallationLite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentEdited {
    pub action: DiscussionCommentEditedAction,
    pub changes: DiscussionCommentEditedChanges,
    pub comment: DiscussionCommentEditedComment,
    pub discussion: Discussion,
    pub installation: InstallationLite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum DiscussionCommentEvent {
    #[doc = "discussion_comment created event"]
    #[serde(rename = "created")]
    Created {
        comment: DiscussionCommentCreatedComment,
        discussion: Discussion,
        installation: InstallationLite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion_comment deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        comment: DiscussionCommentDeletedComment,
        discussion: Discussion,
        installation: InstallationLite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion_comment edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: DiscussionCommentEditedChanges,
        comment: DiscussionCommentEditedComment,
        discussion: Discussion,
        installation: InstallationLite,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum DiscussionEvent {
    #[doc = "discussion answered event"]
    #[serde(rename = "answered")]
    Answered {
        answer: DiscussionAnsweredAnswer,
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion category changed event"]
    #[serde(rename = "category_changed")]
    CategoryChanged {
        changes: DiscussionCategoryChangedChanges,
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion created event"]
    #[serde(rename = "created")]
    Created {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion edited event"]
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<DiscussionEditedChanges>,
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion locked event"]
    #[serde(rename = "locked")]
    Locked {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion pinned event"]
    #[serde(rename = "pinned")]
    Pinned {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion transferred event"]
    #[serde(rename = "transferred")]
    Transferred {
        changes: DiscussionTransferredChanges,
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion unanswered event"]
    #[serde(rename = "unanswered")]
    Unanswered {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        old_answer: DiscussionUnansweredOldAnswer,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion unlocked event"]
    #[serde(rename = "unlocked")]
    Unlocked {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "discussion unpinned event"]
    #[serde(rename = "unpinned")]
    Unpinned {
        discussion: Discussion,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[doc = "A user forks a repository."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ForkEvent {
    #[doc = "The created [`repository`](https://docs.github.com/en/rest/reference/repos#get-a-repository) resource."]
    pub forkee: Repository,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GithubOrg {
    pub avatar_url: String,
    #[serde(default)]
    pub email: (),
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GithubAppAuthorizationRevoked {
    pub action: GithubAppAuthorizationRevokedAction,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubAppAuthorizationEvent(pub GithubAppAuthorizationRevoked);
impl std::ops::Deref for GithubAppAuthorizationEvent {
    type Target = GithubAppAuthorizationRevoked;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A wiki page is created or updated."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GollumEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[doc = "The pages that were updated."]
    pub pages: Vec<GollumEventPagesItem>,
    pub repository: Repository,
    pub sender: User,
}
#[doc = "The GitHub App installation."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Installation {
    pub access_tokens_url: String,
    pub account: User,
    pub app_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_slug: Option<String>,
    pub created_at: InstallationCreatedAt,
    pub events: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_multiple_single_files: Option<bool>,
    pub html_url: String,
    #[doc = "The ID of the installation."]
    pub id: i64,
    pub permissions: InstallationPermissions,
    pub repositories_url: String,
    #[doc = "Describe whether all repositories have been selected or there's a selection involved"]
    pub repository_selection: InstallationRepositorySelection,
    pub single_file_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub single_file_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_by: Option<String>,
    #[doc = "The ID of the user or organization this token is being scoped to."]
    pub target_id: i64,
    pub target_type: InstallationTargetType,
    pub updated_at: InstallationUpdatedAt,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationCreated {
    pub action: InstallationCreatedAction,
    pub installation: Installation,
    #[doc = "An array of repository objects that the installation can access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<InstallationCreatedRepositoriesItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<User>,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationDeleted {
    pub action: InstallationDeletedAction,
    pub installation: Installation,
    #[doc = "An array of repository objects that the installation can access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<InstallationDeletedRepositoriesItem>,
    #[serde(default)]
    pub requester: (),
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationNewPermissionsAccepted {
    pub action: InstallationNewPermissionsAcceptedAction,
    pub installation: Installation,
    #[doc = "An array of repository objects that the installation can access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<InstallationNewPermissionsAcceptedRepositoriesItem>,
    #[serde(default)]
    pub requester: (),
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationSuspend {
    pub action: InstallationSuspendAction,
    pub installation: Installation,
    #[doc = "An array of repository objects that the installation can access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<InstallationSuspendRepositoriesItem>,
    #[serde(default)]
    pub requester: (),
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationUnsuspend {
    pub action: InstallationUnsuspendAction,
    pub installation: Installation,
    #[doc = "An array of repository objects that the installation can access."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub repositories: Vec<InstallationUnsuspendRepositoriesItem>,
    #[serde(default)]
    pub requester: (),
    pub sender: User,
}
#[doc = "Installation"]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationLite {
    #[doc = "The ID of the installation."]
    pub id: i64,
    pub node_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum InstallationEvent {
    #[doc = "installation created event"]
    #[serde(rename = "created")]
    Created {
        installation: Installation,
        #[doc = "An array of repository objects that the installation can access."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationCreatedRepositoriesItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requester: Option<User>,
        sender: User,
    },
    #[doc = "installation deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        installation: Installation,
        #[doc = "An array of repository objects that the installation can access."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationDeletedRepositoriesItem>,
        #[serde(default)]
        requester: (),
        sender: User,
    },
    #[doc = "installation new_permissions_accepted event"]
    #[serde(rename = "new_permissions_accepted")]
    NewPermissionsAccepted {
        installation: Installation,
        #[doc = "An array of repository objects that the installation can access."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationNewPermissionsAcceptedRepositoriesItem>,
        #[serde(default)]
        requester: (),
        sender: User,
    },
    #[doc = "installation suspend event"]
    #[serde(rename = "suspend")]
    Suspend {
        installation: Installation,
        #[doc = "An array of repository objects that the installation can access."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationSuspendRepositoriesItem>,
        #[serde(default)]
        requester: (),
        sender: User,
    },
    #[doc = "installation unsuspend event"]
    #[serde(rename = "unsuspend")]
    Unsuspend {
        installation: Installation,
        #[doc = "An array of repository objects that the installation can access."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        repositories: Vec<InstallationUnsuspendRepositoriesItem>,
        #[serde(default)]
        requester: (),
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesAdded {
    pub action: InstallationRepositoriesAddedAction,
    pub installation: Installation,
    #[doc = "An array of repository objects, which were added to the installation."]
    pub repositories_added: Vec<InstallationRepositoriesAddedRepositoriesAddedItem>,
    #[doc = "An array of repository objects, which were removed from the installation."]
    pub repositories_removed: Vec<InstallationRepositoriesAddedRepositoriesRemovedItem>,
    #[doc = "Describe whether all repositories have been selected or there's a selection involved"]
    pub repository_selection: InstallationRepositoriesAddedRepositorySelection,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<User>,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesRemoved {
    pub action: InstallationRepositoriesRemovedAction,
    pub installation: Installation,
    #[doc = "An array of repository objects, which were added to the installation."]
    pub repositories_added: Vec<InstallationRepositoriesRemovedRepositoriesAddedItem>,
    #[doc = "An array of repository objects, which were removed from the installation."]
    pub repositories_removed: Vec<InstallationRepositoriesRemovedRepositoriesRemovedItem>,
    #[doc = "Describe whether all repositories have been selected or there's a selection involved"]
    pub repository_selection: InstallationRepositoriesRemovedRepositorySelection,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum InstallationRepositoriesEvent {
    #[doc = "installation_repositories added event"]
    #[serde(rename = "added")]
    Added {
        installation: Installation,
        #[doc = "An array of repository objects, which were added to the installation."]
        repositories_added: Vec<InstallationRepositoriesAddedRepositoriesAddedItem>,
        #[doc = "An array of repository objects, which were removed from the installation."]
        repositories_removed: Vec<InstallationRepositoriesAddedRepositoriesRemovedItem>,
        #[doc = "Describe whether all repositories have been selected or there's a selection involved"]
        repository_selection: InstallationRepositoriesAddedRepositorySelection,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        requester: Option<User>,
        sender: User,
    },
    #[doc = "installation_repositories removed event"]
    #[serde(rename = "removed")]
    Removed {
        installation: Installation,
        #[doc = "An array of repository objects, which were added to the installation."]
        repositories_added: Vec<InstallationRepositoriesRemovedRepositoriesAddedItem>,
        #[doc = "An array of repository objects, which were removed from the installation."]
        repositories_removed: Vec<InstallationRepositoriesRemovedRepositoriesRemovedItem>,
        #[doc = "Describe whether all repositories have been selected or there's a selection involved"]
        repository_selection: InstallationRepositoriesRemovedRepositorySelection,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub active_lock_reason: Option<IssueActiveLockReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    #[doc = "Contents of the issue"]
    pub body: String,
    pub closed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub comments: i64,
    pub comments_url: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub events_url: String,
    pub html_url: String,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Label>,
    pub labels_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<App>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<IssuePullRequest>,
    pub repository_url: String,
    #[doc = "State of the issue; either 'open' or 'closed'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<IssueState>,
    #[doc = "Title of the issue"]
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the issue"]
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentCreated {
    pub action: IssueCommentCreatedAction,
    pub comment: IssueCommentCreatedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentDeleted {
    pub action: IssueCommentDeletedAction,
    pub comment: IssueCommentDeletedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentEdited {
    pub action: IssueCommentEditedAction,
    pub changes: IssueCommentEditedChanges,
    pub comment: IssueCommentEditedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum IssueCommentEvent {
    #[doc = "issue_comment created event"]
    #[serde(rename = "created")]
    Created {
        comment: IssueCommentCreatedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issue_comment deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        comment: IssueCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issue_comment edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: IssueCommentEditedChanges,
        comment: IssueCommentEditedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) the comment belongs to."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[doc = "Activity related to an issue. The type of activity is specified in the action property."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesAssigned {
    #[doc = "The action that was performed."]
    pub action: IssuesAssignedAction,
    #[doc = "The optional user who was assigned or unassigned from the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesClosed {
    #[doc = "The action that was performed."]
    pub action: IssuesClosedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesDeleted {
    pub action: IssuesDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesDemilestoned {
    pub action: IssuesDemilestonedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesEdited {
    pub action: IssuesEditedAction,
    pub changes: IssuesEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesLabeled {
    pub action: IssuesLabeledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[doc = "The optional label that was added or removed from the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesLocked {
    pub action: IssuesLockedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesMilestoned {
    pub action: IssuesMilestonedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesOpened {
    pub action: IssuesOpenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesPinned {
    pub action: IssuesPinnedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesReopened {
    pub action: IssuesReopenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesTransferred {
    pub action: IssuesTransferredAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesUnassigned {
    #[doc = "The action that was performed."]
    pub action: IssuesUnassignedAction,
    #[doc = "The optional user who was assigned or unassigned from the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesUnlabeled {
    pub action: IssuesUnlabeledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[doc = "The optional label that was added or removed from the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesUnlocked {
    pub action: IssuesUnlockedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesUnpinned {
    pub action: IssuesUnpinnedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub issue: Issue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum IssuesEvent {
    #[doc = "issues assigned event\n\nActivity related to an issue. The type of activity is specified in the action property."]
    #[serde(rename = "assigned")]
    Assigned {
        #[doc = "The optional user who was assigned or unassigned from the issue."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<User>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues closed event"]
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues demilestoned event"]
    #[serde(rename = "demilestoned")]
    Demilestoned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [issue](https://docs.github.com/en/rest/reference/issues) itself."]
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: IssuesEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues labeled event"]
    #[serde(rename = "labeled")]
    Labeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[doc = "The optional label that was added or removed from the issue."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues locked event"]
    #[serde(rename = "locked")]
    Locked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues milestoned event"]
    #[serde(rename = "milestoned")]
    Milestoned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues opened event"]
    #[serde(rename = "opened")]
    Opened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues pinned event"]
    #[serde(rename = "pinned")]
    Pinned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues reopened event"]
    #[serde(rename = "reopened")]
    Reopened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues transferred event"]
    #[serde(rename = "transferred")]
    Transferred {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues unassigned event"]
    #[serde(rename = "unassigned")]
    Unassigned {
        #[doc = "The optional user who was assigned or unassigned from the issue."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        assignee: Option<User>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues unlabeled event"]
    #[serde(rename = "unlabeled")]
    Unlabeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[doc = "The optional label that was added or removed from the issue."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<Label>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues unlocked event"]
    #[serde(rename = "unlocked")]
    Unlocked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "issues unpinned event"]
    #[serde(rename = "unpinned")]
    Unpinned {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        issue: Issue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Label {
    #[doc = "6-character hex code, without the leading #, identifying the color"]
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
    pub id: i64,
    #[doc = "The name of the label."]
    pub name: String,
    pub node_id: String,
    #[doc = "URL for the label"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelCreated {
    pub action: LabelCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The label that was added."]
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelDeleted {
    pub action: LabelDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The label that was removed."]
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelEdited {
    pub action: LabelEditedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<LabelEditedChanges>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The label that was edited."]
    pub label: Label,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum LabelEvent {
    #[doc = "label created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The label that was added."]
        label: Label,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "label deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The label that was removed."]
        label: Label,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "label edited event"]
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        changes: Option<LabelEditedChanges>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The label that was edited."]
        label: Label,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub key: String,
    pub name: String,
    pub node_id: String,
    pub spdx_id: String,
    pub url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub href: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchase {
    pub account: MarketplacePurchaseAccount,
    pub billing_cycle: String,
    pub free_trial_ends_on: (),
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    pub on_free_trial: bool,
    pub plan: MarketplacePurchasePlan,
    pub unit_count: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchaseCancelled {
    pub action: MarketplacePurchaseCancelledAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: MarketplacePurchaseCancelledSender,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchaseChanged {
    pub action: MarketplacePurchaseChangedAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: MarketplacePurchaseChangedSender,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePendingChange {
    pub action: MarketplacePurchasePendingChangeAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: MarketplacePurchasePendingChangeSender,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePendingChangeCancelled {
    pub action: MarketplacePurchasePendingChangeCancelledAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: MarketplacePurchasePendingChangeCancelledSender,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePurchased {
    pub action: MarketplacePurchasePurchasedAction,
    pub effective_date: String,
    pub marketplace_purchase: MarketplacePurchase,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_marketplace_purchase: Option<MarketplacePurchase>,
    pub sender: MarketplacePurchasePurchasedSender,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum MarketplacePurchaseEvent {
    #[doc = "marketplace_purchase cancelled event"]
    #[serde(rename = "cancelled")]
    Cancelled {
        effective_date: String,
        marketplace_purchase: MarketplacePurchase,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_marketplace_purchase: Option<MarketplacePurchase>,
        sender: MarketplacePurchaseCancelledSender,
    },
    #[doc = "marketplace_purchase changed event"]
    #[serde(rename = "changed")]
    Changed {
        effective_date: String,
        marketplace_purchase: MarketplacePurchase,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_marketplace_purchase: Option<MarketplacePurchase>,
        sender: MarketplacePurchaseChangedSender,
    },
    #[doc = "marketplace_purchase pending_change event"]
    #[serde(rename = "pending_change")]
    PendingChange {
        effective_date: String,
        marketplace_purchase: MarketplacePurchase,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_marketplace_purchase: Option<MarketplacePurchase>,
        sender: MarketplacePurchasePendingChangeSender,
    },
    #[doc = "marketplace_purchase pending_change_cancelled event"]
    #[serde(rename = "pending_change_cancelled")]
    PendingChangeCancelled {
        effective_date: String,
        marketplace_purchase: MarketplacePurchase,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_marketplace_purchase: Option<MarketplacePurchase>,
        sender: MarketplacePurchasePendingChangeCancelledSender,
    },
    #[doc = "marketplace_purchase purchased event"]
    #[serde(rename = "purchased")]
    Purchased {
        effective_date: String,
        marketplace_purchase: MarketplacePurchase,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        previous_marketplace_purchase: Option<MarketplacePurchase>,
        sender: MarketplacePurchasePurchasedSender,
    },
}
#[doc = "Activity related to repository collaborators. The type of activity is specified in the action property."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MemberAdded {
    pub action: MemberAddedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The user that was added."]
    pub member: User,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MemberEdited {
    pub action: MemberEditedAction,
    pub changes: MemberEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The user who's permissions are changed."]
    pub member: User,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MemberRemoved {
    pub action: MemberRemovedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The user that was removed."]
    pub member: User,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum MemberEvent {
    #[doc = "member added event\n\nActivity related to repository collaborators. The type of activity is specified in the action property."]
    #[serde(rename = "added")]
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The user that was added."]
        member: User,
        repository: Repository,
        sender: User,
    },
    #[doc = "member edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: MemberEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The user who's permissions are changed."]
        member: User,
        repository: Repository,
        sender: User,
    },
    #[doc = "member removed event"]
    #[serde(rename = "removed")]
    Removed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The user that was removed."]
        member: User,
        repository: Repository,
        sender: User,
    },
}
#[doc = "The membership between the user and the organization. Not present when the action is `member_invited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Membership {
    pub organization_url: String,
    pub role: String,
    pub state: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MembershipAdded {
    pub action: MembershipAddedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [user](https://docs.github.com/en/rest/reference/users) that was added or removed."]
    pub member: User,
    pub organization: Organization,
    #[doc = "The scope of the membership. Currently, can only be `team`."]
    pub scope: MembershipAddedScope,
    pub sender: User,
    #[doc = "The [team](https://docs.github.com/en/rest/reference/teams) for the membership."]
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MembershipRemoved {
    pub action: MembershipRemovedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The [user](https://docs.github.com/en/rest/reference/users) that was added or removed."]
    pub member: User,
    pub organization: Organization,
    #[doc = "The scope of the membership. Currently, can only be `team`."]
    pub scope: MembershipRemovedScope,
    pub sender: User,
    #[doc = "The [team](https://docs.github.com/en/rest/reference/teams) for the membership."]
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum MembershipEvent {
    #[doc = "membership added event"]
    #[serde(rename = "added")]
    Added {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [user](https://docs.github.com/en/rest/reference/users) that was added or removed."]
        member: User,
        organization: Organization,
        #[doc = "The scope of the membership. Currently, can only be `team`."]
        scope: MembershipAddedScope,
        sender: User,
        #[doc = "The [team](https://docs.github.com/en/rest/reference/teams) for the membership."]
        team: Team,
    },
    #[doc = "membership removed event"]
    #[serde(rename = "removed")]
    Removed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The [user](https://docs.github.com/en/rest/reference/users) that was added or removed."]
        member: User,
        organization: Organization,
        #[doc = "The scope of the membership. Currently, can only be `team`."]
        scope: MembershipRemovedScope,
        sender: User,
        #[doc = "The [team](https://docs.github.com/en/rest/reference/teams) for the membership."]
        team: Team,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetaDeleted {
    pub action: MetaDeletedAction,
    pub hook: MetaDeletedHook,
    #[doc = "The id of the modified webhook."]
    pub hook_id: i64,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaEvent(pub MetaDeleted);
impl std::ops::Deref for MetaEvent {
    type Target = MetaDeleted;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "A collection of related issues and pull requests."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Milestone {
    pub closed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub closed_issues: i64,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub creator: User,
    pub description: Option<String>,
    pub due_on: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub html_url: String,
    pub id: i64,
    pub labels_url: String,
    pub node_id: String,
    #[doc = "The number of the milestone."]
    pub number: i64,
    pub open_issues: i64,
    #[doc = "The state of the milestone."]
    pub state: MilestoneState,
    #[doc = "The title of the milestone."]
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneClosed {
    pub action: MilestoneClosedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub milestone: Milestone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneCreated {
    pub action: MilestoneCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub milestone: Milestone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneDeleted {
    pub action: MilestoneDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub milestone: Milestone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneEdited {
    pub action: MilestoneEditedAction,
    pub changes: MilestoneEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub milestone: Milestone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneOpened {
    pub action: MilestoneOpenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub milestone: Milestone,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum MilestoneEvent {
    #[doc = "milestone closed event"]
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        milestone: Milestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "milestone created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        milestone: Milestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "milestone deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        milestone: Milestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "milestone edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: MilestoneEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        milestone: Milestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "milestone opened event"]
    #[serde(rename = "opened")]
    Opened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        milestone: Milestone,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrgBlockBlocked {
    pub action: OrgBlockBlockedAction,
    #[doc = "Information about the user that was blocked or unblocked."]
    pub blocked_user: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrgBlockUnblocked {
    pub action: OrgBlockUnblockedAction,
    #[doc = "Information about the user that was blocked or unblocked."]
    pub blocked_user: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum OrgBlockEvent {
    #[doc = "org_block blocked event"]
    #[serde(rename = "blocked")]
    Blocked {
        #[doc = "Information about the user that was blocked or unblocked."]
        blocked_user: User,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        organization: Organization,
        sender: User,
    },
    #[doc = "org_block unblocked event"]
    #[serde(rename = "unblocked")]
    Unblocked {
        #[doc = "Information about the user that was blocked or unblocked."]
        blocked_user: User,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        organization: Organization,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Organization {
    pub avatar_url: String,
    pub description: Option<String>,
    pub events_url: String,
    pub hooks_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    pub id: i64,
    pub issues_url: String,
    pub login: String,
    pub members_url: String,
    pub node_id: String,
    pub public_members_url: String,
    pub repos_url: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationDeleted {
    pub action: OrganizationDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub membership: Membership,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationMemberAdded {
    pub action: OrganizationMemberAddedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub membership: Membership,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationMemberInvited {
    pub action: OrganizationMemberInvitedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub invitation: OrganizationMemberInvitedInvitation,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationMemberRemoved {
    pub action: OrganizationMemberRemovedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub membership: Membership,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationRenamed {
    pub action: OrganizationRenamedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub membership: Membership,
    pub organization: Organization,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum OrganizationEvent {
    #[doc = "organization deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        membership: Membership,
        organization: Organization,
        sender: User,
    },
    #[doc = "organization member_added event"]
    #[serde(rename = "member_added")]
    MemberAdded {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        membership: Membership,
        organization: Organization,
        sender: User,
    },
    #[doc = "organization member_invited event"]
    #[serde(rename = "member_invited")]
    MemberInvited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        invitation: OrganizationMemberInvitedInvitation,
        organization: Organization,
        sender: User,
    },
    #[doc = "organization member_removed event"]
    #[serde(rename = "member_removed")]
    MemberRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        membership: Membership,
        organization: Organization,
        sender: User,
    },
    #[doc = "organization renamed event"]
    #[serde(rename = "renamed")]
    Renamed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        membership: Membership,
        organization: Organization,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublished {
    pub action: PackagePublishedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub package: PackagePublishedPackage,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdated {
    pub action: PackageUpdatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub package: PackageUpdatedPackage,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum PackageEvent {
    #[doc = "package published event"]
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        package: PackagePublishedPackage,
        repository: Repository,
        sender: User,
    },
    #[doc = "package updated event"]
    #[serde(rename = "updated")]
    Updated {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        package: PackageUpdatedPackage,
        repository: Repository,
        sender: User,
    },
}
#[doc = "Page Build"]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PageBuildEvent {
    pub build: PageBuildEventBuild,
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PingEvent {
    pub hook: PingEventHook,
    pub hook_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender: Option<User>,
    pub zen: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Project {
    #[doc = "Body of the project"]
    pub body: Option<String>,
    pub columns_url: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub creator: User,
    pub html_url: String,
    pub id: i64,
    #[doc = "Name of the project"]
    pub name: String,
    pub node_id: String,
    pub number: i64,
    pub owner_url: String,
    #[doc = "State of the project; either 'open' or 'closed'"]
    pub state: ProjectState,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectClosed {
    pub action: ProjectClosedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project: Project,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCreated {
    pub action: ProjectCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project: Project,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectDeleted {
    pub action: ProjectDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project: Project,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectEdited {
    pub action: ProjectEditedAction,
    pub changes: ProjectEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project: Project,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectReopened {
    pub action: ProjectReopenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project: Project,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCard {
    #[serde(default)]
    pub after_id: (),
    #[doc = "Whether or not the card is archived"]
    pub archived: bool,
    pub column_id: i64,
    pub column_url: String,
    pub content_url: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub creator: User,
    #[doc = "The project card's ID"]
    pub id: i64,
    pub node_id: String,
    pub note: Option<String>,
    pub project_url: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectColumn {
    pub cards_url: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The unique identifier of the project column"]
    pub id: i64,
    #[doc = "Name of the project column"]
    pub name: String,
    pub node_id: String,
    pub project_url: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardConverted {
    pub action: ProjectCardConvertedAction,
    pub changes: ProjectCardConvertedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_card: ProjectCard,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardCreated {
    pub action: ProjectCardCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_card: ProjectCard,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardDeleted {
    pub action: ProjectCardDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_card: ProjectCard,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardEdited {
    pub action: ProjectCardEditedAction,
    pub changes: ProjectCardEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_card: ProjectCard,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardMoved {
    pub action: ProjectCardMovedAction,
    pub changes: ProjectCardMovedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_card: ProjectCard,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum ProjectCardEvent {
    #[doc = "project_card converted event"]
    #[serde(rename = "converted")]
    Converted {
        changes: ProjectCardConvertedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_card: ProjectCard,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_card created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_card: ProjectCard,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_card deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_card: ProjectCard,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_card edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: ProjectCardEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_card: ProjectCard,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_card moved event"]
    #[serde(rename = "moved")]
    Moved {
        changes: ProjectCardMovedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_card: ProjectCard,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectColumnCreated {
    pub action: ProjectColumnCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_column: ProjectColumn,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectColumnDeleted {
    pub action: ProjectColumnDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_column: ProjectColumn,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectColumnEdited {
    pub action: ProjectColumnEditedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_column: ProjectColumn,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectColumnMoved {
    pub action: ProjectColumnMovedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub project_column: ProjectColumn,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum ProjectColumnEvent {
    #[doc = "project_column created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_column: ProjectColumn,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_column deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_column: ProjectColumn,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_column edited event"]
    #[serde(rename = "edited")]
    Edited {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_column: ProjectColumn,
        repository: Repository,
        sender: User,
    },
    #[doc = "project_column moved event"]
    #[serde(rename = "moved")]
    Moved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project_column: ProjectColumn,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum ProjectEvent {
    #[doc = "project closed event"]
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project: Project,
        repository: Repository,
        sender: User,
    },
    #[doc = "project created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project: Project,
        repository: Repository,
        sender: User,
    },
    #[doc = "project deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project: Project,
        repository: Repository,
        sender: User,
    },
    #[doc = "project edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: ProjectEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project: Project,
        repository: Repository,
        sender: User,
    },
    #[doc = "project reopened event"]
    #[serde(rename = "reopened")]
    Reopened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        project: Project,
        repository: Repository,
        sender: User,
    },
}
#[doc = "When a private repository is made public."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PublicEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequest {
    pub active_lock_reason: Option<PullRequestActiveLockReason>,
    pub additions: i64,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    pub auto_merge: (),
    pub base: PullRequestBase,
    pub body: String,
    pub changed_files: i64,
    pub closed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub comments: i64,
    pub comments_url: String,
    pub commits: i64,
    pub commits_url: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub deletions: i64,
    pub diff_url: String,
    #[doc = "Indicates whether or not the pull request is a draft."]
    pub draft: bool,
    pub head: PullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<Label>,
    #[serde(rename = "_links")]
    pub links: PullRequestLinks,
    pub locked: bool,
    #[doc = "Indicates whether maintainers can modify the pull request."]
    pub maintainer_can_modify: bool,
    pub merge_commit_sha: Option<String>,
    pub mergeable: Option<bool>,
    pub mergeable_state: String,
    pub merged: Option<bool>,
    pub merged_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub merged_by: Option<User>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    #[doc = "Number uniquely identifying the pull request within its repository."]
    pub number: i64,
    pub patch_url: String,
    pub rebaseable: Option<bool>,
    pub requested_reviewers: Vec<PullRequestRequestedReviewersItem>,
    pub requested_teams: Vec<Team>,
    pub review_comment_url: String,
    pub review_comments: i64,
    pub review_comments_url: String,
    #[doc = "State of this Pull Request. Either `open` or `closed`."]
    pub state: PullRequestState,
    pub statuses_url: String,
    #[doc = "The title of the pull request."]
    pub title: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestAssigned {
    pub action: PullRequestAssignedAction,
    pub assignee: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestAutoMergeDisabled {
    pub action: PullRequestAutoMergeDisabledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestAutoMergeEnabled {
    pub action: PullRequestAutoMergeEnabledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestClosed {
    pub action: PullRequestClosedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestConvertedToDraft {
    pub action: PullRequestConvertedToDraftAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestEdited {
    pub action: PullRequestEditedAction,
    pub changes: PullRequestEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestLabeled {
    pub action: PullRequestLabeledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub label: Label,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestLocked {
    pub action: PullRequestLockedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestOpened {
    pub action: PullRequestOpenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReadyForReview {
    pub action: PullRequestReadyForReviewAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReopened {
    pub action: PullRequestReopenedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewRequestRemoved {
    pub action: PullRequestReviewRequestRemovedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub requested_reviewer: User,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewRequested {
    pub action: PullRequestReviewRequestedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub requested_reviewer: User,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestSynchronize {
    pub action: PullRequestSynchronizeAction,
    pub after: String,
    pub before: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestUnassigned {
    pub action: PullRequestUnassignedAction,
    pub assignee: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestUnlabeled {
    pub action: PullRequestUnlabeledAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub label: Label,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestUnlocked {
    pub action: PullRequestUnlockedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[doc = "The pull request number."]
    pub number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum PullRequestEvent {
    #[doc = "pull_request assigned event"]
    #[serde(rename = "assigned")]
    Assigned {
        assignee: User,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request auto_merge_disabled event"]
    #[serde(rename = "auto_merge_disabled")]
    AutoMergeDisabled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request auto_merge_enabled event"]
    #[serde(rename = "auto_merge_enabled")]
    AutoMergeEnabled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request closed event"]
    #[serde(rename = "closed")]
    Closed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request converted_to_draft event"]
    #[serde(rename = "converted_to_draft")]
    ConvertedToDraft {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: PullRequestEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request labeled event"]
    #[serde(rename = "labeled")]
    Labeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        label: Label,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request locked event"]
    #[serde(rename = "locked")]
    Locked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request opened event"]
    #[serde(rename = "opened")]
    Opened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request ready_for_review event"]
    #[serde(rename = "ready_for_review")]
    ReadyForReview {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request reopened event"]
    #[serde(rename = "reopened")]
    Reopened {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request review_request_removed event"]
    #[serde(rename = "review_request_removed")]
    ReviewRequestRemoved {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        requested_reviewer: User,
        sender: User,
    },
    #[doc = "pull_request review_requested event"]
    #[serde(rename = "review_requested")]
    ReviewRequested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        requested_reviewer: User,
        sender: User,
    },
    #[doc = "pull_request synchronize event"]
    #[serde(rename = "synchronize")]
    Synchronize {
        after: String,
        before: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request unassigned event"]
    #[serde(rename = "unassigned")]
    Unassigned {
        assignee: User,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request unlabeled event"]
    #[serde(rename = "unlabeled")]
    Unlabeled {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        label: Label,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request unlocked event"]
    #[serde(rename = "unlocked")]
    Unlocked {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[doc = "The pull request number."]
        number: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequest,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewDismissed {
    pub action: PullRequestReviewDismissedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: SimplePullRequest,
    pub repository: Repository,
    pub review: PullRequestReviewDismissedReview,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewEdited {
    pub action: PullRequestReviewEditedAction,
    pub changes: PullRequestReviewEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: SimplePullRequest,
    pub repository: Repository,
    pub review: PullRequestReviewEditedReview,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewSubmitted {
    pub action: PullRequestReviewSubmittedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: SimplePullRequest,
    pub repository: Repository,
    pub review: PullRequestReviewSubmittedReview,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreated {
    pub action: PullRequestReviewCommentCreatedAction,
    pub comment: PullRequestReviewCommentCreatedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequestReviewCommentCreatedPullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeleted {
    pub action: PullRequestReviewCommentDeletedAction,
    pub comment: PullRequestReviewCommentDeletedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequestReviewCommentDeletedPullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEdited {
    pub action: PullRequestReviewCommentEditedAction,
    pub changes: PullRequestReviewCommentEditedChanges,
    pub comment: PullRequestReviewCommentEditedComment,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pull_request: PullRequestReviewCommentEditedPullRequest,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum PullRequestReviewCommentEvent {
    #[doc = "pull_request_review_comment created event"]
    #[serde(rename = "created")]
    Created {
        comment: PullRequestReviewCommentCreatedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequestReviewCommentCreatedPullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request_review_comment deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        comment: PullRequestReviewCommentDeletedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequestReviewCommentDeletedPullRequest,
        repository: Repository,
        sender: User,
    },
    #[doc = "pull_request_review_comment edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: PullRequestReviewCommentEditedChanges,
        comment: PullRequestReviewCommentEditedComment,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: PullRequestReviewCommentEditedPullRequest,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum PullRequestReviewEvent {
    #[doc = "pull_request_review dismissed event"]
    #[serde(rename = "dismissed")]
    Dismissed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: SimplePullRequest,
        repository: Repository,
        review: PullRequestReviewDismissedReview,
        sender: User,
    },
    #[doc = "pull_request_review edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: PullRequestReviewEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: SimplePullRequest,
        repository: Repository,
        review: PullRequestReviewEditedReview,
        sender: User,
    },
    #[doc = "pull_request_review submitted event"]
    #[serde(rename = "submitted")]
    Submitted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        pull_request: SimplePullRequest,
        repository: Repository,
        review: PullRequestReviewSubmittedReview,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PushEvent {
    #[doc = "The SHA of the most recent commit on `ref` after the push."]
    pub after: String,
    pub base_ref: (),
    #[doc = "The SHA of the most recent commit on `ref` before the push."]
    pub before: String,
    #[doc = "An array of commit objects describing the pushed commits."]
    pub commits: Vec<Commit>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Option<Commit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub pusher: Committer,
    #[doc = "The full git ref that was pushed. Example: `refs/heads/main`."]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseCreated {
    pub action: ReleaseCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleaseCreatedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseDeleted {
    pub action: ReleaseDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleaseDeletedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseEdited {
    pub action: ReleaseEditedAction,
    pub changes: ReleaseEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleaseEditedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleasePrereleased {
    pub action: ReleasePrereleasedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleasePrereleasedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleasePublished {
    pub action: ReleasePublishedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleasePublishedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseReleased {
    pub action: ReleaseReleasedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleaseReleasedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseUnpublished {
    pub action: ReleaseUnpublishedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub release: ReleaseUnpublishedRelease,
    pub repository: Repository,
    pub sender: User,
}
#[doc = "Data related to a release."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseAsset {
    pub browser_download_url: String,
    pub content_type: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub download_count: i64,
    pub id: i64,
    pub label: String,
    #[doc = "The file name of the asset."]
    pub name: String,
    pub node_id: String,
    pub size: i64,
    #[doc = "State of the release asset."]
    pub state: ReleaseAssetState,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploader: Option<User>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum ReleaseEvent {
    #[doc = "release created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleaseCreatedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleaseDeletedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: ReleaseEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleaseEditedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release prereleased event"]
    #[serde(rename = "prereleased")]
    Prereleased {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleasePrereleasedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release published event"]
    #[serde(rename = "published")]
    Published {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleasePublishedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release released event"]
    #[serde(rename = "released")]
    Released {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleaseReleasedRelease,
        repository: Repository,
        sender: User,
    },
    #[doc = "release unpublished event"]
    #[serde(rename = "unpublished")]
    Unpublished {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        release: ReleaseUnpublishedRelease,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepoRef {
    pub id: i64,
    pub name: String,
    pub url: String,
}
#[doc = "A git repository"]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    #[doc = "Whether to allow merge commits for pull requests."]
    #[serde(default = "defaults::default_bool::<false>")]
    pub allow_merge_commit: bool,
    #[doc = "Whether to allow rebase merges for pull requests."]
    #[serde(default = "defaults::default_bool::<false>")]
    pub allow_rebase_merge: bool,
    #[doc = "Whether to allow squash merges for pull requests."]
    #[serde(default = "defaults::default_bool::<false>")]
    pub allow_squash_merge: bool,
    pub archive_url: String,
    #[doc = "Whether the repository is archived."]
    pub archived: bool,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: RepositoryCreatedAt,
    #[doc = "The default branch of the repository."]
    pub default_branch: String,
    #[doc = "Whether to delete head branches when pull requests are merged"]
    #[serde(default)]
    pub delete_branch_on_merge: bool,
    pub deployments_url: String,
    pub description: Option<String>,
    #[doc = "Returns whether or not this repository is disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks: i64,
    pub forks_count: i64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    #[doc = "Whether downloads are enabled."]
    pub has_downloads: bool,
    #[doc = "Whether issues are enabled."]
    pub has_issues: bool,
    pub has_pages: bool,
    #[doc = "Whether projects are enabled."]
    pub has_projects: bool,
    #[doc = "Whether the wiki is enabled."]
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub license: Option<License>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master_branch: Option<String>,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub open_issues: i64,
    pub open_issues_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    pub owner: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RepositoryPermissions>,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    pub pulls_url: String,
    pub pushed_at: RepositoryPushedAt,
    pub releases_url: String,
    pub size: i64,
    pub ssh_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stargazers: Option<i64>,
    pub stargazers_count: i64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
    pub watchers: i64,
    pub watchers_count: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryArchived {
    pub action: RepositoryArchivedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryCreated {
    pub action: RepositoryCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDeleted {
    pub action: RepositoryDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEdited {
    pub action: RepositoryEditedAction,
    pub changes: RepositoryEditedChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryPrivatized {
    pub action: RepositoryPrivatizedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryPublicized {
    pub action: RepositoryPublicizedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryRenamed {
    pub action: RepositoryRenamedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryTransferred {
    pub action: RepositoryTransferredAction,
    pub changes: RepositoryTransferredChanges,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryUnarchived {
    pub action: RepositoryUnarchivedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryLite {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub hooks_url: String,
    pub html_url: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    pub notifications_url: String,
    pub owner: User,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
    pub pulls_url: String,
    pub releases_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDispatchOnDemandTest {
    pub action: RepositoryDispatchOnDemandTestAction,
    pub branch: String,
    pub client_payload: std::collections::HashMap<String, serde_json::Value>,
    pub installation: InstallationLite,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepositoryDispatchEvent(pub RepositoryDispatchOnDemandTest);
impl std::ops::Deref for RepositoryDispatchEvent {
    type Target = RepositoryDispatchOnDemandTest;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum RepositoryEvent {
    #[doc = "repository archived event"]
    #[serde(rename = "archived")]
    Archived {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: RepositoryEditedChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository privatized event"]
    #[serde(rename = "privatized")]
    Privatized {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository publicized event"]
    #[serde(rename = "publicized")]
    Publicized {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository renamed event"]
    #[serde(rename = "renamed")]
    Renamed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository transferred event"]
    #[serde(rename = "transferred")]
    Transferred {
        changes: RepositoryTransferredChanges,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository unarchived event"]
    #[serde(rename = "unarchived")]
    Unarchived {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryImportEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    pub status: RepositoryImportEventStatus,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertCreate {
    pub action: RepositoryVulnerabilityAlertCreateAction,
    pub alert: RepositoryVulnerabilityAlertCreateAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertDismiss {
    pub action: RepositoryVulnerabilityAlertDismissAction,
    pub alert: RepositoryVulnerabilityAlertDismissAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertResolve {
    pub action: RepositoryVulnerabilityAlertResolveAction,
    pub alert: RepositoryVulnerabilityAlertResolveAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum RepositoryVulnerabilityAlertEvent {
    #[doc = "repository_vulnerability_alert create event"]
    #[serde(rename = "create")]
    Create {
        alert: RepositoryVulnerabilityAlertCreateAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository_vulnerability_alert dismiss event"]
    #[serde(rename = "dismiss")]
    Dismiss {
        alert: RepositoryVulnerabilityAlertDismissAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "repository_vulnerability_alert resolve event"]
    #[serde(rename = "resolve")]
    Resolve {
        alert: RepositoryVulnerabilityAlertResolveAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertCreated {
    pub action: SecretScanningAlertCreatedAction,
    pub alert: SecretScanningAlertCreatedAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertReopened {
    pub action: SecretScanningAlertReopenedAction,
    pub alert: SecretScanningAlertReopenedAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertResolved {
    pub action: SecretScanningAlertResolvedAction,
    pub alert: SecretScanningAlertResolvedAlert,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum SecretScanningAlertEvent {
    #[doc = "secret_scanning_alert created event"]
    #[serde(rename = "created")]
    Created {
        alert: SecretScanningAlertCreatedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
    },
    #[doc = "secret_scanning_alert reopened event"]
    #[serde(rename = "reopened")]
    Reopened {
        alert: SecretScanningAlertReopenedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
    #[doc = "secret_scanning_alert resolved event"]
    #[serde(rename = "resolved")]
    Resolved {
        alert: SecretScanningAlertResolvedAlert,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformed {
    pub action: SecurityAdvisoryPerformedAction,
    pub security_advisory: SecurityAdvisoryPerformedSecurityAdvisory,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublished {
    pub action: SecurityAdvisoryPublishedAction,
    pub security_advisory: SecurityAdvisoryPublishedSecurityAdvisory,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdated {
    pub action: SecurityAdvisoryUpdatedAction,
    pub security_advisory: SecurityAdvisoryUpdatedSecurityAdvisory,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", content = "security_advisory")]
pub enum SecurityAdvisoryEvent {
    #[doc = "security_advisory performed event"]
    #[serde(rename = "performed")]
    Performed(SecurityAdvisoryPerformedSecurityAdvisory),
    #[doc = "security_advisory published event"]
    #[serde(rename = "published")]
    Published(SecurityAdvisoryPublishedSecurityAdvisory),
    #[doc = "security_advisory updated event"]
    #[serde(rename = "updated")]
    Updated(SecurityAdvisoryUpdatedSecurityAdvisory),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SimplePullRequest {
    pub active_lock_reason: Option<SimplePullRequestActiveLockReason>,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    pub base: SimplePullRequestBase,
    pub body: String,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub draft: bool,
    pub head: SimplePullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<Label>,
    #[serde(rename = "_links")]
    pub links: SimplePullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<SimplePullRequestRequestedReviewersItem>,
    pub requested_teams: Vec<Team>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: SimplePullRequestState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipCancelled {
    pub action: SponsorshipCancelledAction,
    pub sender: User,
    pub sponsorship: SponsorshipCancelledSponsorship,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipCreated {
    pub action: SponsorshipCreatedAction,
    pub sender: User,
    pub sponsorship: SponsorshipCreatedSponsorship,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipEdited {
    pub action: SponsorshipEditedAction,
    pub changes: SponsorshipEditedChanges,
    pub sender: User,
    pub sponsorship: SponsorshipEditedSponsorship,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingCancellation {
    pub action: SponsorshipPendingCancellationAction,
    #[doc = "The `pending_cancellation` and `pending_tier_change` event types will include the date the cancellation or tier change will take effect."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    pub sender: User,
    pub sponsorship: SponsorshipPendingCancellationSponsorship,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingTierChange {
    pub action: SponsorshipPendingTierChangeAction,
    pub changes: SponsorshipPendingTierChangeChanges,
    #[doc = "The `pending_cancellation` and `pending_tier_change` event types will include the date the cancellation or tier change will take effect."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    pub sender: User,
    pub sponsorship: SponsorshipPendingTierChangeSponsorship,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipTierChanged {
    pub action: SponsorshipTierChangedAction,
    pub changes: SponsorshipTierChangedChanges,
    pub sender: User,
    pub sponsorship: SponsorshipTierChangedSponsorship,
}
#[doc = "The `tier_changed` and `pending_tier_change` will include the original tier before the change or pending change. For more information, see the pending tier change payload."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipTier {
    pub created_at: String,
    pub description: String,
    pub is_custom_ammount: bool,
    pub is_one_time: bool,
    pub monthly_price_in_cents: i64,
    pub monthly_price_in_dollars: i64,
    pub name: String,
    pub node_id: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum SponsorshipEvent {
    #[doc = "sponsorship cancelled event"]
    #[serde(rename = "cancelled")]
    Cancelled {
        sender: User,
        sponsorship: SponsorshipCancelledSponsorship,
    },
    #[doc = "sponsorship created event"]
    #[serde(rename = "created")]
    Created {
        sender: User,
        sponsorship: SponsorshipCreatedSponsorship,
    },
    #[doc = "sponsorship edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: SponsorshipEditedChanges,
        sender: User,
        sponsorship: SponsorshipEditedSponsorship,
    },
    #[doc = "sponsorship pending_cancellation event"]
    #[serde(rename = "pending_cancellation")]
    PendingCancellation {
        #[doc = "The `pending_cancellation` and `pending_tier_change` event types will include the date the cancellation or tier change will take effect."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        effective_date: Option<String>,
        sender: User,
        sponsorship: SponsorshipPendingCancellationSponsorship,
    },
    #[doc = "sponsorship pending_tier_change event"]
    #[serde(rename = "pending_tier_change")]
    PendingTierChange {
        changes: SponsorshipPendingTierChangeChanges,
        #[doc = "The `pending_cancellation` and `pending_tier_change` event types will include the date the cancellation or tier change will take effect."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        effective_date: Option<String>,
        sender: User,
        sponsorship: SponsorshipPendingTierChangeSponsorship,
    },
    #[doc = "sponsorship tier_changed event"]
    #[serde(rename = "tier_changed")]
    TierChanged {
        changes: SponsorshipTierChangedChanges,
        sender: User,
        sponsorship: SponsorshipTierChangedSponsorship,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StarCreated {
    pub action: StarCreatedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    #[doc = "The time the star was created. This is a timestamp in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. Will be `null` for the `deleted` action."]
    pub starred_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StarDeleted {
    pub action: StarDeletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    #[doc = "The time the star was created. This is a timestamp in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. Will be `null` for the `deleted` action."]
    pub starred_at: (),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum StarEvent {
    #[doc = "star created event"]
    #[serde(rename = "created")]
    Created {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
        #[doc = "The time the star was created. This is a timestamp in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. Will be `null` for the `deleted` action."]
        starred_at: String,
    },
    #[doc = "star deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
        #[doc = "The time the star was created. This is a timestamp in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`. Will be `null` for the `deleted` action."]
        starred_at: (),
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[doc = "An array of branch objects containing the status' SHA. Each branch contains the given SHA, but the SHA may or may not be the head of the branch. The array includes a maximum of 10 branches."]
    pub branches: Vec<StatusEventBranchesItem>,
    pub commit: StatusEventCommit,
    pub context: String,
    pub created_at: String,
    #[doc = "The optional human-readable description added to the status."]
    pub description: Option<String>,
    #[doc = "The unique identifier of the status."]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    #[doc = "The Commit SHA."]
    pub sha: String,
    #[doc = "The new state. Can be `pending`, `success`, `failure`, or `error`."]
    pub state: StatusEventState,
    #[doc = "The optional link added to the status."]
    pub target_url: Option<String>,
    pub updated_at: String,
}
#[doc = "Groups of organization members that gives permissions on specified repositories."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Team {
    #[doc = "Description of the team"]
    pub description: Option<String>,
    pub html_url: String,
    #[doc = "Unique identifier of the team"]
    pub id: i64,
    pub members_url: String,
    #[doc = "Name of the team"]
    pub name: String,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<TeamParent>,
    #[doc = "Permission that the team will have for its repositories"]
    pub permission: String,
    pub privacy: TeamPrivacy,
    pub repositories_url: String,
    pub slug: String,
    #[doc = "URL for the team"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamAddedToRepository {
    pub action: TeamAddedToRepositoryAction,
    pub organization: Organization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamCreated {
    pub action: TeamCreatedAction,
    pub organization: Organization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamDeleted {
    pub action: TeamDeletedAction,
    pub organization: Organization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEdited {
    pub action: TeamEditedAction,
    pub changes: TeamEditedChanges,
    pub organization: Organization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamRemovedFromRepository {
    pub action: TeamRemovedFromRepositoryAction,
    pub organization: Organization,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamAddEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    pub organization: Organization,
    pub repository: Repository,
    pub sender: User,
    pub team: Team,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum TeamEvent {
    #[doc = "team added_to_repository event"]
    #[serde(rename = "added_to_repository")]
    AddedToRepository {
        organization: Organization,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Repository>,
        sender: User,
        team: Team,
    },
    #[doc = "team created event"]
    #[serde(rename = "created")]
    Created {
        organization: Organization,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Repository>,
        sender: User,
        team: Team,
    },
    #[doc = "team deleted event"]
    #[serde(rename = "deleted")]
    Deleted {
        organization: Organization,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Repository>,
        sender: User,
        team: Team,
    },
    #[doc = "team edited event"]
    #[serde(rename = "edited")]
    Edited {
        changes: TeamEditedChanges,
        organization: Organization,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Repository>,
        sender: User,
        team: Team,
    },
    #[doc = "team removed_from_repository event"]
    #[serde(rename = "removed_from_repository")]
    RemovedFromRepository {
        organization: Organization,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        repository: Option<Repository>,
        sender: User,
        team: Team,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub avatar_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: UserType,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WatchStarted {
    pub action: WatchStartedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WatchEvent(pub WatchStarted);
impl std::ops::Deref for WatchEvent {
    type Target = WatchStarted;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
    pub badge_url: String,
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub path: String,
    pub state: String,
    pub updated_at: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorkflowRun {
    pub artifacts_url: String,
    pub cancel_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_suite_id: Option<i64>,
    pub check_suite_url: String,
    pub conclusion: Option<String>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub event: String,
    pub head_branch: String,
    pub head_commit: CommitSimple,
    pub head_repository: RepositoryLite,
    pub head_sha: String,
    pub html_url: String,
    pub id: i64,
    pub jobs_url: String,
    pub logs_url: String,
    pub name: String,
    pub node_id: String,
    pub pull_requests: Vec<PullRequest>,
    pub repository: RepositoryLite,
    pub rerun_url: String,
    pub run_number: i64,
    pub status: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
    pub workflow_id: i64,
    pub workflow_url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorkflowDispatchEvent {
    pub inputs: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository: Repository,
    pub sender: User,
    pub workflow: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorkflowRunCompleted {
    pub action: WorkflowRunCompletedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    pub workflow: Workflow,
    pub workflow_run: WorkflowRun,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorkflowRunRequested {
    pub action: WorkflowRunRequestedAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installation: Option<InstallationLite>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub repository: Repository,
    pub sender: User,
    pub workflow: Workflow,
    pub workflow_run: WorkflowRun,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action", deny_unknown_fields)]
pub enum WorkflowRunEvent {
    #[doc = "workflow_run completed event"]
    #[serde(rename = "completed")]
    Completed {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
        workflow: Workflow,
        workflow_run: WorkflowRun,
    },
    #[doc = "workflow_run requested event"]
    #[serde(rename = "requested")]
    Requested {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        installation: Option<InstallationLite>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        organization: Option<Organization>,
        repository: Repository,
        sender: User,
        workflow: Workflow,
        workflow_run: WorkflowRun,
    },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AlertInstanceLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AlertInstanceMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AlertInstanceState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
}
impl ToString for AlertInstanceState {
    fn to_string(&self) -> String {
        match *self {
            AlertInstanceState::Open => "open".to_string(),
            AlertInstanceState::Dismissed => "dismissed".to_string(),
            AlertInstanceState::Fixed => "fixed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsActions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsActions {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsActions::Read => "read".to_string(),
            AppPermissionsActions::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsAdministration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsAdministration {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsAdministration::Read => "read".to_string(),
            AppPermissionsAdministration::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsChecks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsChecks {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsChecks::Read => "read".to_string(),
            AppPermissionsChecks::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsContents {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsContents {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsContents::Read => "read".to_string(),
            AppPermissionsContents::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsDeployments {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsDeployments {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsDeployments::Read => "read".to_string(),
            AppPermissionsDeployments::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsEmails {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsEmails {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsEmails::Read => "read".to_string(),
            AppPermissionsEmails::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsIssues {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsIssues {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsIssues::Read => "read".to_string(),
            AppPermissionsIssues::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsMembers {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsMembers {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsMembers::Read => "read".to_string(),
            AppPermissionsMembers::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsMetadata {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsMetadata {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsMetadata::Read => "read".to_string(),
            AppPermissionsMetadata::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationAdministration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationAdministration {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationAdministration::Read => "read".to_string(),
            AppPermissionsOrganizationAdministration::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationHooks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationHooks {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationHooks::Read => "read".to_string(),
            AppPermissionsOrganizationHooks::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationPackages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationPackages {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationPackages::Read => "read".to_string(),
            AppPermissionsOrganizationPackages::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationPlan {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationPlan {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationPlan::Read => "read".to_string(),
            AppPermissionsOrganizationPlan::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationProjects {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationProjects {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationProjects::Read => "read".to_string(),
            AppPermissionsOrganizationProjects::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsOrganizationUserBlocking {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsOrganizationUserBlocking {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsOrganizationUserBlocking::Read => "read".to_string(),
            AppPermissionsOrganizationUserBlocking::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsPackages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsPackages {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsPackages::Read => "read".to_string(),
            AppPermissionsPackages::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsPages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsPages {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsPages::Read => "read".to_string(),
            AppPermissionsPages::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsPullRequests {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsPullRequests {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsPullRequests::Read => "read".to_string(),
            AppPermissionsPullRequests::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsRepositoryHooks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsRepositoryHooks {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsRepositoryHooks::Read => "read".to_string(),
            AppPermissionsRepositoryHooks::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsRepositoryProjects {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsRepositoryProjects {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsRepositoryProjects::Read => "read".to_string(),
            AppPermissionsRepositoryProjects::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsSecurityEvents {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsSecurityEvents {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsSecurityEvents::Read => "read".to_string(),
            AppPermissionsSecurityEvents::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsStatuses {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsStatuses {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsStatuses::Read => "read".to_string(),
            AppPermissionsStatuses::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsTeamDiscussions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsTeamDiscussions {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsTeamDiscussions::Read => "read".to_string(),
            AppPermissionsTeamDiscussions::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum AppPermissionsVulnerabilityAlerts {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for AppPermissionsVulnerabilityAlerts {
    fn to_string(&self) -> String {
        match *self {
            AppPermissionsVulnerabilityAlerts::Read => "read".to_string(),
            AppPermissionsVulnerabilityAlerts::Write => "write".to_string(),
        }
    }
}
#[doc = "The set of permissions for the GitHub app"]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AppPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<AppPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<AppPermissionsAdministration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<AppPermissionsChecks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<AppPermissionsContents>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<AppPermissionsDeployments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<AppPermissionsEmails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<AppPermissionsIssues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<AppPermissionsMembers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AppPermissionsMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<AppPermissionsOrganizationAdministration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_hooks: Option<AppPermissionsOrganizationHooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_packages: Option<AppPermissionsOrganizationPackages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_plan: Option<AppPermissionsOrganizationPlan>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_projects: Option<AppPermissionsOrganizationProjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_user_blocking: Option<AppPermissionsOrganizationUserBlocking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<AppPermissionsPackages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<AppPermissionsPages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<AppPermissionsPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<AppPermissionsRepositoryHooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<AppPermissionsRepositoryProjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security_events: Option<AppPermissionsSecurityEvents>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<AppPermissionsStatuses>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_discussions: Option<AppPermissionsTeamDiscussions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<AppPermissionsVulnerabilityAlerts>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunPullRequestBase {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: RepoRef,
    pub sha: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunPullRequestHead {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: RepoRef,
    pub sha: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCompletedAction {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunCompletedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCompletedAction::Completed => "completed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCompletedCheckRunCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunCompletedCheckRunCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCompletedCheckRunCheckSuiteConclusion::Success => "success".to_string(),
            CheckRunCompletedCheckRunCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckRunCompletedCheckRunCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckRunCompletedCheckRunCheckSuiteConclusion::Cancelled => "cancelled".to_string(),
            CheckRunCompletedCheckRunCheckSuiteConclusion::TimedOut => "timed_out".to_string(),
            CheckRunCompletedCheckRunCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckRunCompletedCheckRunCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCompletedCheckRunCheckSuiteStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunCompletedCheckRunCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCompletedCheckRunCheckSuiteStatus::InProgress => "in_progress".to_string(),
            CheckRunCompletedCheckRunCheckSuiteStatus::Completed => "completed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCompletedCheckRunCheckSuite {
    pub after: Option<String>,
    pub app: App,
    pub before: Option<String>,
    pub conclusion: Option<CheckRunCompletedCheckRunCheckSuiteConclusion>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub head_branch: Option<String>,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    #[doc = "The id of the check suite that this check run is part of."]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    pub status: CheckRunCompletedCheckRunCheckSuiteStatus,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCompletedCheckRunConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunCompletedCheckRunConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCompletedCheckRunConclusion::Success => "success".to_string(),
            CheckRunCompletedCheckRunConclusion::Failure => "failure".to_string(),
            CheckRunCompletedCheckRunConclusion::Neutral => "neutral".to_string(),
            CheckRunCompletedCheckRunConclusion::Cancelled => "cancelled".to_string(),
            CheckRunCompletedCheckRunConclusion::TimedOut => "timed_out".to_string(),
            CheckRunCompletedCheckRunConclusion::ActionRequired => "action_required".to_string(),
            CheckRunCompletedCheckRunConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCompletedCheckRunOutput {
    pub annotations_count: i64,
    pub annotations_url: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCompletedCheckRunStatus {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunCompletedCheckRunStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCompletedCheckRunStatus::Completed => "completed".to_string(),
        }
    }
}
#[doc = "The [check_run](https://docs.github.com/en/rest/reference/checks#get-a-check-run)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCompletedCheckRun {
    pub app: App,
    pub check_suite: CheckRunCompletedCheckRunCheckSuite,
    #[doc = "The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub completed_at: String,
    #[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
    pub conclusion: Option<CheckRunCompletedCheckRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<CheckRunDeployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    pub external_id: String,
    #[doc = "The SHA of the commit that is being checked."]
    pub head_sha: String,
    pub html_url: String,
    #[doc = "The id of the check."]
    pub id: i64,
    #[doc = "The name of the check run."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub output: CheckRunCompletedCheckRunOutput,
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub started_at: String,
    #[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
    pub status: CheckRunCompletedCheckRunStatus,
    pub url: String,
}
#[doc = "The action requested by the user."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCompletedRequestedAction {
    #[doc = "The integrator reference of the action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for CheckRunCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCreatedCheckRunCheckSuiteStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ToString for CheckRunCreatedCheckRunCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCreatedCheckRunCheckSuiteStatus::Queued => "queued".to_string(),
            CheckRunCreatedCheckRunCheckSuiteStatus::InProgress => "in_progress".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCreatedCheckRunCheckSuite {
    pub after: Option<String>,
    pub app: App,
    pub before: Option<String>,
    pub conclusion: (),
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub head_branch: Option<String>,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    #[doc = "The id of the check suite that this check run is part of."]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    pub status: CheckRunCreatedCheckRunCheckSuiteStatus,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCreatedCheckRunConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunCreatedCheckRunConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCreatedCheckRunConclusion::Success => "success".to_string(),
            CheckRunCreatedCheckRunConclusion::Failure => "failure".to_string(),
            CheckRunCreatedCheckRunConclusion::Neutral => "neutral".to_string(),
            CheckRunCreatedCheckRunConclusion::Cancelled => "cancelled".to_string(),
            CheckRunCreatedCheckRunConclusion::TimedOut => "timed_out".to_string(),
            CheckRunCreatedCheckRunConclusion::ActionRequired => "action_required".to_string(),
            CheckRunCreatedCheckRunConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCreatedCheckRunOutput {
    pub annotations_count: i64,
    pub annotations_url: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunCreatedCheckRunStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
}
impl ToString for CheckRunCreatedCheckRunStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunCreatedCheckRunStatus::Queued => "queued".to_string(),
            CheckRunCreatedCheckRunStatus::InProgress => "in_progress".to_string(),
        }
    }
}
#[doc = "The [check_run](https://docs.github.com/en/rest/reference/checks#get-a-check-run)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCreatedCheckRun {
    pub app: App,
    pub check_suite: CheckRunCreatedCheckRunCheckSuite,
    #[doc = "The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub completed_at: Option<String>,
    #[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
    pub conclusion: Option<CheckRunCreatedCheckRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<CheckRunDeployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    pub external_id: String,
    #[doc = "The SHA of the commit that is being checked."]
    pub head_sha: String,
    pub html_url: String,
    #[doc = "The id of the check."]
    pub id: i64,
    #[doc = "The name of the check run."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub output: CheckRunCreatedCheckRunOutput,
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub started_at: String,
    #[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
    pub status: CheckRunCreatedCheckRunStatus,
    pub url: String,
}
#[doc = "The action requested by the user."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunCreatedRequestedAction {
    #[doc = "The integrator reference of the action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRequestedActionAction {
    #[serde(rename = "requested_action")]
    RequestedAction,
}
impl ToString for CheckRunRequestedActionAction {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRequestedActionAction::RequestedAction => "requested_action".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRequestedActionCheckRunCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunRequestedActionCheckRunCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::Success => "success".to_string(),
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::Cancelled => {
                "cancelled".to_string()
            }
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::TimedOut => {
                "timed_out".to_string()
            }
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckRunRequestedActionCheckRunCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRequestedActionCheckRunCheckSuiteStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunRequestedActionCheckRunCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRequestedActionCheckRunCheckSuiteStatus::Queued => "queued".to_string(),
            CheckRunRequestedActionCheckRunCheckSuiteStatus::InProgress => {
                "in_progress".to_string()
            }
            CheckRunRequestedActionCheckRunCheckSuiteStatus::Completed => "completed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRequestedActionCheckRunCheckSuite {
    pub after: Option<String>,
    pub app: App,
    pub before: Option<String>,
    pub conclusion: Option<CheckRunRequestedActionCheckRunCheckSuiteConclusion>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub head_branch: Option<String>,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    #[doc = "The id of the check suite that this check run is part of."]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    pub status: CheckRunRequestedActionCheckRunCheckSuiteStatus,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRequestedActionCheckRunConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunRequestedActionCheckRunConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRequestedActionCheckRunConclusion::Success => "success".to_string(),
            CheckRunRequestedActionCheckRunConclusion::Failure => "failure".to_string(),
            CheckRunRequestedActionCheckRunConclusion::Neutral => "neutral".to_string(),
            CheckRunRequestedActionCheckRunConclusion::Cancelled => "cancelled".to_string(),
            CheckRunRequestedActionCheckRunConclusion::TimedOut => "timed_out".to_string(),
            CheckRunRequestedActionCheckRunConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckRunRequestedActionCheckRunConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRequestedActionCheckRunOutput {
    pub annotations_count: i64,
    pub annotations_url: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRequestedActionCheckRunStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunRequestedActionCheckRunStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRequestedActionCheckRunStatus::Queued => "queued".to_string(),
            CheckRunRequestedActionCheckRunStatus::InProgress => "in_progress".to_string(),
            CheckRunRequestedActionCheckRunStatus::Completed => "completed".to_string(),
        }
    }
}
#[doc = "The [check_run](https://docs.github.com/en/rest/reference/checks#get-a-check-run)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRequestedActionCheckRun {
    pub app: App,
    pub check_suite: CheckRunRequestedActionCheckRunCheckSuite,
    #[doc = "The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub completed_at: Option<String>,
    #[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
    pub conclusion: Option<CheckRunRequestedActionCheckRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<CheckRunDeployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    pub external_id: String,
    #[doc = "The SHA of the commit that is being checked."]
    pub head_sha: String,
    pub html_url: String,
    #[doc = "The id of the check."]
    pub id: i64,
    #[doc = "The name of the check run."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub output: CheckRunRequestedActionCheckRunOutput,
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub started_at: String,
    #[doc = "The current status of the check run. Can be `queued`, `in_progress`, or `completed`."]
    pub status: CheckRunRequestedActionCheckRunStatus,
    pub url: String,
}
#[doc = "The action requested by the user."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRequestedActionRequestedAction {
    #[doc = "The integrator reference of the action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRerequestedAction {
    #[serde(rename = "rerequested")]
    Rerequested,
}
impl ToString for CheckRunRerequestedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRerequestedAction::Rerequested => "rerequested".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRerequestedCheckRunCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunRerequestedCheckRunCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRerequestedCheckRunCheckSuiteConclusion::Success => "success".to_string(),
            CheckRunRerequestedCheckRunCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckRunRerequestedCheckRunCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckRunRerequestedCheckRunCheckSuiteConclusion::Cancelled => "cancelled".to_string(),
            CheckRunRerequestedCheckRunCheckSuiteConclusion::TimedOut => "timed_out".to_string(),
            CheckRunRerequestedCheckRunCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckRunRerequestedCheckRunCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRerequestedCheckRunCheckSuiteStatus {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunRerequestedCheckRunCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRerequestedCheckRunCheckSuiteStatus::Completed => "completed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRerequestedCheckRunCheckSuite {
    pub after: Option<String>,
    pub app: App,
    pub before: Option<String>,
    pub conclusion: CheckRunRerequestedCheckRunCheckSuiteConclusion,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub head_branch: Option<String>,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    #[doc = "The id of the check suite that this check run is part of."]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    pub status: CheckRunRerequestedCheckRunCheckSuiteStatus,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub url: String,
}
#[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRerequestedCheckRunConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckRunRerequestedCheckRunConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRerequestedCheckRunConclusion::Success => "success".to_string(),
            CheckRunRerequestedCheckRunConclusion::Failure => "failure".to_string(),
            CheckRunRerequestedCheckRunConclusion::Neutral => "neutral".to_string(),
            CheckRunRerequestedCheckRunConclusion::Cancelled => "cancelled".to_string(),
            CheckRunRerequestedCheckRunConclusion::TimedOut => "timed_out".to_string(),
            CheckRunRerequestedCheckRunConclusion::ActionRequired => "action_required".to_string(),
            CheckRunRerequestedCheckRunConclusion::Stale => "stale".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRerequestedCheckRunOutput {
    pub annotations_count: i64,
    pub annotations_url: String,
    pub summary: Option<String>,
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[doc = "The phase of the lifecycle that the check is currently in."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckRunRerequestedCheckRunStatus {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckRunRerequestedCheckRunStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckRunRerequestedCheckRunStatus::Completed => "completed".to_string(),
        }
    }
}
#[doc = "The [check_run](https://docs.github.com/en/rest/reference/checks#get-a-check-run)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRerequestedCheckRun {
    pub app: App,
    pub check_suite: CheckRunRerequestedCheckRunCheckSuite,
    #[doc = "The time the check completed. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub completed_at: String,
    #[doc = "The result of the completed check run. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has `completed`."]
    pub conclusion: Option<CheckRunRerequestedCheckRunConclusion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment: Option<CheckRunDeployment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,
    pub external_id: String,
    #[doc = "The SHA of the commit that is being checked."]
    pub head_sha: String,
    pub html_url: String,
    #[doc = "The id of the check."]
    pub id: i64,
    #[doc = "The name of the check."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub output: CheckRunRerequestedCheckRunOutput,
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The time that the check run began. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub started_at: String,
    #[doc = "The phase of the lifecycle that the check is currently in."]
    pub status: CheckRunRerequestedCheckRunStatus,
    pub url: String,
}
#[doc = "The action requested by the user."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckRunRerequestedRequestedAction {
    #[doc = "The integrator reference of the action requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteCompletedAction {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for CheckSuiteCompletedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteCompletedAction::Completed => "completed".to_string(),
        }
    }
}
#[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteCompletedCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckSuiteCompletedCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteCompletedCheckSuiteConclusion::Success => "success".to_string(),
            CheckSuiteCompletedCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckSuiteCompletedCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckSuiteCompletedCheckSuiteConclusion::Cancelled => "cancelled".to_string(),
            CheckSuiteCompletedCheckSuiteConclusion::TimedOut => "timed_out".to_string(),
            CheckSuiteCompletedCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckSuiteCompletedCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteCompletedCheckSuiteStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "queued")]
    Queued,
}
impl ToString for CheckSuiteCompletedCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteCompletedCheckSuiteStatus::Requested => "requested".to_string(),
            CheckSuiteCompletedCheckSuiteStatus::InProgress => "in_progress".to_string(),
            CheckSuiteCompletedCheckSuiteStatus::Completed => "completed".to_string(),
            CheckSuiteCompletedCheckSuiteStatus::Queued => "queued".to_string(),
        }
    }
}
#[doc = "The [check_suite](https://docs.github.com/en/rest/reference/checks#suites)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteCompletedCheckSuite {
    pub after: String,
    pub app: App,
    pub before: String,
    pub check_runs_url: String,
    #[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`, `neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has `completed`."]
    pub conclusion: Option<CheckSuiteCompletedCheckSuiteConclusion>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The head branch name the changes are on."]
    pub head_branch: Option<String>,
    pub head_commit: CommitSimple,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
    pub status: Option<CheckSuiteCompletedCheckSuiteStatus>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL that points to the check suite API resource."]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRequestedAction {
    #[serde(rename = "requested")]
    Requested,
}
impl ToString for CheckSuiteRequestedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRequestedAction::Requested => "requested".to_string(),
        }
    }
}
#[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`,` neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRequestedCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckSuiteRequestedCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRequestedCheckSuiteConclusion::Success => "success".to_string(),
            CheckSuiteRequestedCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckSuiteRequestedCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckSuiteRequestedCheckSuiteConclusion::Cancelled => "cancelled".to_string(),
            CheckSuiteRequestedCheckSuiteConclusion::TimedOut => "timed_out".to_string(),
            CheckSuiteRequestedCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckSuiteRequestedCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRequestedCheckSuiteStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "queued")]
    Queued,
}
impl ToString for CheckSuiteRequestedCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRequestedCheckSuiteStatus::Requested => "requested".to_string(),
            CheckSuiteRequestedCheckSuiteStatus::InProgress => "in_progress".to_string(),
            CheckSuiteRequestedCheckSuiteStatus::Completed => "completed".to_string(),
            CheckSuiteRequestedCheckSuiteStatus::Queued => "queued".to_string(),
        }
    }
}
#[doc = "The [check_suite](https://docs.github.com/en/rest/reference/checks#suites)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteRequestedCheckSuite {
    pub after: String,
    pub app: App,
    pub before: String,
    pub check_runs_url: String,
    #[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`,` neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
    pub conclusion: Option<CheckSuiteRequestedCheckSuiteConclusion>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The head branch name the changes are on."]
    pub head_branch: Option<String>,
    pub head_commit: CommitSimple,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
    pub status: Option<CheckSuiteRequestedCheckSuiteStatus>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL that points to the check suite API resource."]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRerequestedAction {
    #[serde(rename = "rerequested")]
    Rerequested,
}
impl ToString for CheckSuiteRerequestedAction {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRerequestedAction::Rerequested => "rerequested".to_string(),
        }
    }
}
#[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`,` neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRerequestedCheckSuiteConclusion {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "timed_out")]
    TimedOut,
    #[serde(rename = "action_required")]
    ActionRequired,
    #[serde(rename = "stale")]
    Stale,
}
impl ToString for CheckSuiteRerequestedCheckSuiteConclusion {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRerequestedCheckSuiteConclusion::Success => "success".to_string(),
            CheckSuiteRerequestedCheckSuiteConclusion::Failure => "failure".to_string(),
            CheckSuiteRerequestedCheckSuiteConclusion::Neutral => "neutral".to_string(),
            CheckSuiteRerequestedCheckSuiteConclusion::Cancelled => "cancelled".to_string(),
            CheckSuiteRerequestedCheckSuiteConclusion::TimedOut => "timed_out".to_string(),
            CheckSuiteRerequestedCheckSuiteConclusion::ActionRequired => {
                "action_required".to_string()
            }
            CheckSuiteRerequestedCheckSuiteConclusion::Stale => "stale".to_string(),
        }
    }
}
#[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CheckSuiteRerequestedCheckSuiteStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "queued")]
    Queued,
}
impl ToString for CheckSuiteRerequestedCheckSuiteStatus {
    fn to_string(&self) -> String {
        match *self {
            CheckSuiteRerequestedCheckSuiteStatus::Requested => "requested".to_string(),
            CheckSuiteRerequestedCheckSuiteStatus::InProgress => "in_progress".to_string(),
            CheckSuiteRerequestedCheckSuiteStatus::Completed => "completed".to_string(),
            CheckSuiteRerequestedCheckSuiteStatus::Queued => "queued".to_string(),
        }
    }
}
#[doc = "The [check_suite](https://docs.github.com/en/rest/reference/checks#suites)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CheckSuiteRerequestedCheckSuite {
    pub after: String,
    pub app: App,
    pub before: String,
    pub check_runs_url: String,
    #[doc = "The summary conclusion for all check runs that are part of the check suite. Can be one of `success`, `failure`,` neutral`, `cancelled`, `timed_out`, `action_required` or `stale`. This value will be `null` until the check run has completed."]
    pub conclusion: Option<CheckSuiteRerequestedCheckSuiteConclusion>,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The head branch name the changes are on."]
    pub head_branch: Option<String>,
    pub head_commit: CommitSimple,
    #[doc = "The SHA of the head commit that is being checked."]
    pub head_sha: String,
    pub id: i64,
    pub latest_check_runs_count: i64,
    pub node_id: String,
    #[doc = "An array of pull requests that match this check suite. A pull request matches a check suite if they have the same `head_sha` and `head_branch`. When the check suite's `head_branch` is in a forked repository it will be `null` and the `pull_requests` array will be empty."]
    pub pull_requests: Vec<CheckRunPullRequest>,
    #[doc = "The summary status for all check runs that are part of the check suite. Can be `requested`, `in_progress`, or `completed`."]
    pub status: Option<CheckSuiteRerequestedCheckSuiteStatus>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL that points to the check suite API resource."]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertAppearedInBranchAction {
    #[serde(rename = "appeared_in_branch")]
    AppearedInBranch,
}
impl ToString for CodeScanningAlertAppearedInBranchAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertAppearedInBranchAction::AppearedInBranch => {
                "appeared_in_branch".to_string()
            }
        }
    }
}
#[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertAppearedInBranchAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WontFix,
    #[serde(rename = "used in tests")]
    UsedInTests,
}
impl ToString for CodeScanningAlertAppearedInBranchAlertDismissedReason {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertAppearedInBranchAlertDismissedReason::FalsePositive => {
                "false positive".to_string()
            }
            CodeScanningAlertAppearedInBranchAlertDismissedReason::WontFix => {
                "won't fix".to_string()
            }
            CodeScanningAlertAppearedInBranchAlertDismissedReason::UsedInTests => {
                "used in tests".to_string()
            }
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertAppearedInBranchAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertAppearedInBranchAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertAppearedInBranchAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertAppearedInBranchAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertAppearedInBranchAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertAppearedInBranchAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertAppearedInBranchAlertRuleSeverity>,
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertAppearedInBranchAlertState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
}
impl ToString for CodeScanningAlertAppearedInBranchAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertAppearedInBranchAlertState::Open => "open".to_string(),
            CodeScanningAlertAppearedInBranchAlertState::Dismissed => "dismissed".to_string(),
            CodeScanningAlertAppearedInBranchAlertState::Fixed => "fixed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertAppearedInBranchAlertTool {
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertAppearedInBranchAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub dismissed_by: Option<User>,
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: Option<CodeScanningAlertAppearedInBranchAlertDismissedReason>,
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertAppearedInBranchAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertAppearedInBranchAlertState,
    pub tool: CodeScanningAlertAppearedInBranchAlertTool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertClosedByUserAction {
    #[serde(rename = "closed_by_user")]
    ClosedByUser,
}
impl ToString for CodeScanningAlertClosedByUserAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertClosedByUserAction::ClosedByUser => "closed_by_user".to_string(),
        }
    }
}
#[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertClosedByUserAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WontFix,
    #[serde(rename = "used in tests")]
    UsedInTests,
}
impl ToString for CodeScanningAlertClosedByUserAlertDismissedReason {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertClosedByUserAlertDismissedReason::FalsePositive => {
                "false positive".to_string()
            }
            CodeScanningAlertClosedByUserAlertDismissedReason::WontFix => "won't fix".to_string(),
            CodeScanningAlertClosedByUserAlertDismissedReason::UsedInTests => {
                "used in tests".to_string()
            }
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertClosedByUserAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertClosedByUserAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertClosedByUserAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertClosedByUserAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertClosedByUserAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertClosedByUserAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertClosedByUserAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(default)]
    pub help: (),
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertClosedByUserAlertRuleSeverity>,
    #[serde(default)]
    pub tags: (),
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertClosedByUserAlertState {
    #[serde(rename = "dismissed")]
    Dismissed,
}
impl ToString for CodeScanningAlertClosedByUserAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertClosedByUserAlertState::Dismissed => "dismissed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertClosedByUserAlertTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertClosedByUserAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: chrono::DateTime<chrono::offset::Utc>,
    pub dismissed_by: User,
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: Option<CodeScanningAlertClosedByUserAlertDismissedReason>,
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertClosedByUserAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertClosedByUserAlertState,
    pub tool: CodeScanningAlertClosedByUserAlertTool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for CodeScanningAlertCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertCreatedAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertCreatedAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertCreatedAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertCreatedAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertCreatedAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertCreatedAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertCreatedAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(default)]
    pub help: (),
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertCreatedAlertRuleSeverity>,
    #[serde(default)]
    pub tags: (),
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertCreatedAlertState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
}
impl ToString for CodeScanningAlertCreatedAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertCreatedAlertState::Open => "open".to_string(),
            CodeScanningAlertCreatedAlertState::Dismissed => "dismissed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertCreatedAlertTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertCreatedAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: (),
    pub dismissed_by: (),
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: (),
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertCreatedAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertCreatedAlertState,
    pub tool: CodeScanningAlertCreatedAlertTool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertFixedAction {
    #[serde(rename = "fixed")]
    Fixed,
}
impl ToString for CodeScanningAlertFixedAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertFixedAction::Fixed => "fixed".to_string(),
        }
    }
}
#[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertFixedAlertDismissedReason {
    #[serde(rename = "false positive")]
    FalsePositive,
    #[serde(rename = "won't fix")]
    WontFix,
    #[serde(rename = "used in tests")]
    UsedInTests,
}
impl ToString for CodeScanningAlertFixedAlertDismissedReason {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertFixedAlertDismissedReason::FalsePositive => {
                "false positive".to_string()
            }
            CodeScanningAlertFixedAlertDismissedReason::WontFix => "won't fix".to_string(),
            CodeScanningAlertFixedAlertDismissedReason::UsedInTests => "used in tests".to_string(),
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertFixedAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertFixedAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertFixedAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertFixedAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertFixedAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertFixedAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertFixedAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(default)]
    pub help: (),
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertFixedAlertRuleSeverity>,
    #[serde(default)]
    pub tags: (),
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertFixedAlertState {
    #[serde(rename = "fixed")]
    Fixed,
}
impl ToString for CodeScanningAlertFixedAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertFixedAlertState::Fixed => "fixed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertFixedAlertTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertFixedAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub dismissed_by: Option<User>,
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: Option<CodeScanningAlertFixedAlertDismissedReason>,
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_instance: Option<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertFixedAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertFixedAlertState,
    pub tool: CodeScanningAlertFixedAlertTool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedAction {
    #[serde(rename = "reopened")]
    Reopened,
}
impl ToString for CodeScanningAlertReopenedAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedAction::Reopened => "reopened".to_string(),
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertReopenedAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertReopenedAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertReopenedAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertReopenedAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    #[serde(default)]
    pub help: (),
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertReopenedAlertRuleSeverity>,
    #[serde(default)]
    pub tags: (),
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedAlertState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "dismissed")]
    Dismissed,
    #[serde(rename = "fixed")]
    Fixed,
}
impl ToString for CodeScanningAlertReopenedAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedAlertState::Open => "open".to_string(),
            CodeScanningAlertReopenedAlertState::Dismissed => "dismissed".to_string(),
            CodeScanningAlertReopenedAlertState::Fixed => "fixed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedAlertTool {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: (),
    pub dismissed_by: (),
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: (),
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertReopenedAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertReopenedAlertState,
    pub tool: CodeScanningAlertReopenedAlertTool,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedByUserAction {
    #[serde(rename = "reopened_by_user")]
    ReopenedByUser,
}
impl ToString for CodeScanningAlertReopenedByUserAction {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedByUserAction::ReopenedByUser => "reopened_by_user".to_string(),
        }
    }
}
#[doc = "The severity of the alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedByUserAlertRuleSeverity {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}
impl ToString for CodeScanningAlertReopenedByUserAlertRuleSeverity {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedByUserAlertRuleSeverity::None => "none".to_string(),
            CodeScanningAlertReopenedByUserAlertRuleSeverity::Note => "note".to_string(),
            CodeScanningAlertReopenedByUserAlertRuleSeverity::Warning => "warning".to_string(),
            CodeScanningAlertReopenedByUserAlertRuleSeverity::Error => "error".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedByUserAlertRule {
    #[doc = "A short description of the rule used to detect the alert."]
    pub description: String,
    #[doc = "A unique identifier for the rule used to detect the alert."]
    pub id: String,
    #[doc = "The severity of the alert."]
    pub severity: Option<CodeScanningAlertReopenedByUserAlertRuleSeverity>,
}
#[doc = "State of a code scanning alert."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CodeScanningAlertReopenedByUserAlertState {
    #[serde(rename = "open")]
    Open,
}
impl ToString for CodeScanningAlertReopenedByUserAlertState {
    fn to_string(&self) -> String {
        match *self {
            CodeScanningAlertReopenedByUserAlertState::Open => "open".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedByUserAlertTool {
    #[doc = "The name of the tool used to generate the code scanning analysis alert."]
    pub name: String,
    #[doc = "The version of the tool used to detect the alert."]
    pub version: Option<String>,
}
#[doc = "The code scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CodeScanningAlertReopenedByUserAlert {
    #[doc = "The time that the alert was created in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ.`"]
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The time that the alert was dismissed in ISO 8601 format: `YYYY-MM-DDTHH:MM:SSZ`."]
    pub dismissed_at: (),
    pub dismissed_by: (),
    #[doc = "The reason for dismissing or closing the alert. Can be one of: `false positive`, `won't fix`, and `used in tests`."]
    pub dismissed_reason: (),
    #[doc = "The GitHub URL of the alert resource."]
    pub html_url: String,
    pub instances: Vec<AlertInstance>,
    #[doc = "The code scanning alert number."]
    pub number: i64,
    pub rule: CodeScanningAlertReopenedByUserAlertRule,
    #[doc = "State of a code scanning alert."]
    pub state: CodeScanningAlertReopenedByUserAlertState,
    pub tool: CodeScanningAlertReopenedByUserAlertTool,
    pub url: String,
}
#[doc = "The action performed. Can be `created`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CommitCommentCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for CommitCommentCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            CommitCommentCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The [commit comment](https://docs.github.com/en/rest/reference/repos#get-a-commit-comment) resource."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CommitCommentCreatedComment {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the comment."]
    pub body: String,
    #[doc = "The SHA of the commit to which the comment applies."]
    pub commit_id: String,
    pub created_at: String,
    pub html_url: String,
    #[doc = "The ID of the commit comment."]
    pub id: i64,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    pub line: Option<i64>,
    #[doc = "The node ID of the commit comment."]
    pub node_id: String,
    #[doc = "The relative path of the file to which the comment applies."]
    pub path: Option<String>,
    #[doc = "The line index in the diff to which the comment applies."]
    pub position: Option<i64>,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ContentReferenceCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for ContentReferenceCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            ContentReferenceCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ContentReferenceCreatedContentReference {
    pub id: i64,
    pub node_id: String,
    pub reference: String,
}
#[doc = "The type of Git ref object created in the repository. Can be either `branch` or `tag`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum CreateEventRefType {
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "branch")]
    Branch,
}
impl ToString for CreateEventRefType {
    fn to_string(&self) -> String {
        match *self {
            CreateEventRefType::Tag => "tag".to_string(),
            CreateEventRefType::Branch => "branch".to_string(),
        }
    }
}
#[doc = "The type of Git ref object deleted in the repository. Can be either `branch` or `tag`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DeleteEventRefType {
    #[serde(rename = "tag")]
    Tag,
    #[serde(rename = "branch")]
    Branch,
}
impl ToString for DeleteEventRefType {
    fn to_string(&self) -> String {
        match *self {
            DeleteEventRefType::Tag => "tag".to_string(),
            DeleteEventRefType::Branch => "branch".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DeployKeyCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for DeployKeyCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            DeployKeyCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The [`deploy key`](https://docs.github.com/en/rest/reference/repos#get-a-deploy-key) resource."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeployKeyCreatedKey {
    pub created_at: String,
    pub id: i64,
    pub key: String,
    pub read_only: bool,
    pub title: String,
    pub url: String,
    pub verified: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DeployKeyDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for DeployKeyDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            DeployKeyDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[doc = "The [`deploy key`](https://docs.github.com/en/rest/reference/repos#get-a-deploy-key) resource."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeployKeyDeletedKey {
    pub created_at: String,
    pub id: i64,
    pub key: String,
    pub read_only: bool,
    pub title: String,
    pub url: String,
    pub verified: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DeploymentCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for DeploymentCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            DeploymentCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentCreatedDeploymentPayload {}
#[doc = "The [deployment](https://docs.github.com/en/rest/reference/repos#list-deployments)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentCreatedDeployment {
    pub created_at: String,
    pub creator: User,
    pub description: (),
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    pub original_environment: String,
    pub payload: DeploymentCreatedDeploymentPayload,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<App>,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    pub updated_at: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DeploymentStatusCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for DeploymentStatusCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            DeploymentStatusCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentStatusCreatedDeploymentPayload {}
#[doc = "The [deployment](https://docs.github.com/en/rest/reference/repos#list-deployments) that this status is associated with."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentStatusCreatedDeployment {
    pub created_at: String,
    pub creator: User,
    pub description: (),
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    pub original_environment: String,
    pub payload: DeploymentStatusCreatedDeploymentPayload,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    pub updated_at: String,
    pub url: String,
}
#[doc = "The [deployment status](https://docs.github.com/en/rest/reference/repos#list-deployment-statuses)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DeploymentStatusCreatedDeploymentStatus {
    pub created_at: String,
    pub creator: User,
    pub deployment_url: String,
    #[doc = "The optional human-readable description added to the status."]
    pub description: String,
    pub environment: String,
    pub id: i64,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<App>,
    pub repository_url: String,
    #[doc = "The new state. Can be `pending`, `success`, `failure`, or `error`."]
    pub state: String,
    #[doc = "The optional link added to the status."]
    pub target_url: String,
    pub updated_at: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCategory {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: String,
    pub emoji: String,
    pub id: i64,
    pub is_answerable: bool,
    pub name: String,
    pub repository_id: i64,
    pub slug: String,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "locked")]
    Locked,
}
impl ToString for DiscussionState {
    fn to_string(&self) -> String {
        match *self {
            DiscussionState::Open => "open".to_string(),
            DiscussionState::Locked => "locked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionAnsweredAction {
    #[serde(rename = "answered")]
    Answered,
}
impl ToString for DiscussionAnsweredAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionAnsweredAction::Answered => "answered".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionAnsweredAnswer {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: (),
    pub repository_url: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionCategoryChangedAction {
    #[serde(rename = "category_changed")]
    CategoryChanged,
}
impl ToString for DiscussionCategoryChangedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionCategoryChangedAction::CategoryChanged => "category_changed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCategoryChangedChangesCategoryFrom {
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub description: String,
    pub emoji: String,
    pub id: i64,
    pub is_answerable: bool,
    pub name: String,
    pub repository_id: i64,
    pub slug: String,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCategoryChangedChangesCategory {
    pub from: DiscussionCategoryChangedChangesCategoryFrom,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCategoryChangedChanges {
    pub category: DiscussionCategoryChangedChangesCategory,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for DiscussionCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for DiscussionDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for DiscussionEditedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionEditedChangesBody {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionEditedChangesTitle {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<DiscussionEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<DiscussionEditedChangesTitle>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionLockedAction {
    #[serde(rename = "locked")]
    Locked,
}
impl ToString for DiscussionLockedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionLockedAction::Locked => "locked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionPinnedAction {
    #[serde(rename = "pinned")]
    Pinned,
}
impl ToString for DiscussionPinnedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionPinnedAction::Pinned => "pinned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionTransferredAction {
    #[serde(rename = "transferred")]
    Transferred,
}
impl ToString for DiscussionTransferredAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionTransferredAction::Transferred => "transferred".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionTransferredChanges {
    pub new_discussion: Discussion,
    pub new_repository: Repository,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionUnansweredAction {
    #[serde(rename = "unanswered")]
    Unanswered,
}
impl ToString for DiscussionUnansweredAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionUnansweredAction::Unanswered => "unanswered".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionUnansweredOldAnswer {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: (),
    pub repository_url: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionUnlockedAction {
    #[serde(rename = "unlocked")]
    Unlocked,
}
impl ToString for DiscussionUnlockedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionUnlockedAction::Unlocked => "unlocked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionUnpinnedAction {
    #[serde(rename = "unpinned")]
    Unpinned,
}
impl ToString for DiscussionUnpinnedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionUnpinnedAction::Unpinned => "unpinned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionCommentCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for DiscussionCommentCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionCommentCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentCreatedComment {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: String,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: Option<i64>,
    pub repository_url: String,
    pub updated_at: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionCommentDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for DiscussionCommentDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionCommentDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentDeletedComment {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: String,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: Option<i64>,
    pub repository_url: String,
    pub updated_at: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum DiscussionCommentEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for DiscussionCommentEditedAction {
    fn to_string(&self) -> String {
        match *self {
            DiscussionCommentEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentEditedChangesBody {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentEditedChanges {
    pub body: DiscussionCommentEditedChangesBody,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct DiscussionCommentEditedComment {
    pub author_association: AuthorAssociation,
    pub body: String,
    pub child_comment_count: i64,
    pub created_at: String,
    pub discussion_id: i64,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub parent_id: Option<i64>,
    pub repository_url: String,
    pub updated_at: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum GithubAppAuthorizationRevokedAction {
    #[serde(rename = "revoked")]
    Revoked,
}
impl ToString for GithubAppAuthorizationRevokedAction {
    fn to_string(&self) -> String {
        match *self {
            GithubAppAuthorizationRevokedAction::Revoked => "revoked".to_string(),
        }
    }
}
#[doc = "The action that was performed on the page. Can be `created` or `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum GollumEventPagesItemAction {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for GollumEventPagesItemAction {
    fn to_string(&self) -> String {
        match *self {
            GollumEventPagesItemAction::Created => "created".to_string(),
            GollumEventPagesItemAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GollumEventPagesItem {
    #[doc = "The action that was performed on the page. Can be `created` or `edited`."]
    pub action: GollumEventPagesItemAction,
    #[doc = "Points to the HTML wiki page."]
    pub html_url: String,
    #[doc = "The name of the page."]
    pub page_name: String,
    #[doc = "The latest commit SHA of the page."]
    pub sha: String,
    pub summary: (),
    #[doc = "The current page title."]
    pub title: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InstallationCreatedAt {
    Variant0(chrono::DateTime<chrono::offset::Utc>),
    Variant1(i64),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsActions {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsActions {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsActions::Read => "read".to_string(),
            InstallationPermissionsActions::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsAdministration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsAdministration {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsAdministration::Read => "read".to_string(),
            InstallationPermissionsAdministration::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsChecks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsChecks {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsChecks::Read => "read".to_string(),
            InstallationPermissionsChecks::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsContents {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsContents {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsContents::Read => "read".to_string(),
            InstallationPermissionsContents::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsDeployments {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsDeployments {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsDeployments::Read => "read".to_string(),
            InstallationPermissionsDeployments::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsIssues {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsIssues {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsIssues::Read => "read".to_string(),
            InstallationPermissionsIssues::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsMembers {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsMembers {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsMembers::Read => "read".to_string(),
            InstallationPermissionsMembers::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsMetadata {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsMetadata {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsMetadata::Read => "read".to_string(),
            InstallationPermissionsMetadata::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsOrganizationAdministration {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsOrganizationAdministration {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsOrganizationAdministration::Read => "read".to_string(),
            InstallationPermissionsOrganizationAdministration::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsPackages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsPackages {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsPackages::Read => "read".to_string(),
            InstallationPermissionsPackages::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsPages {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsPages {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsPages::Read => "read".to_string(),
            InstallationPermissionsPages::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsPullRequests {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsPullRequests {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsPullRequests::Read => "read".to_string(),
            InstallationPermissionsPullRequests::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsRepositoryHooks {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsRepositoryHooks {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsRepositoryHooks::Read => "read".to_string(),
            InstallationPermissionsRepositoryHooks::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsRepositoryProjects {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsRepositoryProjects {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsRepositoryProjects::Read => "read".to_string(),
            InstallationPermissionsRepositoryProjects::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsStatuses {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsStatuses {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsStatuses::Read => "read".to_string(),
            InstallationPermissionsStatuses::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationPermissionsVulnerabilityAlerts {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}
impl ToString for InstallationPermissionsVulnerabilityAlerts {
    fn to_string(&self) -> String {
        match *self {
            InstallationPermissionsVulnerabilityAlerts::Read => "read".to_string(),
            InstallationPermissionsVulnerabilityAlerts::Write => "write".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<InstallationPermissionsActions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administration: Option<InstallationPermissionsAdministration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checks: Option<InstallationPermissionsChecks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<InstallationPermissionsContents>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployments: Option<InstallationPermissionsDeployments>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issues: Option<InstallationPermissionsIssues>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<InstallationPermissionsMembers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<InstallationPermissionsMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization_administration: Option<InstallationPermissionsOrganizationAdministration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<InstallationPermissionsPackages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages: Option<InstallationPermissionsPages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_requests: Option<InstallationPermissionsPullRequests>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_hooks: Option<InstallationPermissionsRepositoryHooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_projects: Option<InstallationPermissionsRepositoryProjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statuses: Option<InstallationPermissionsStatuses>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vulnerability_alerts: Option<InstallationPermissionsVulnerabilityAlerts>,
}
#[doc = "Describe whether all repositories have been selected or there's a selection involved"]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
}
impl ToString for InstallationRepositorySelection {
    fn to_string(&self) -> String {
        match *self {
            InstallationRepositorySelection::All => "all".to_string(),
            InstallationRepositorySelection::Selected => "selected".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationTargetType {
    User,
    Organization,
}
impl ToString for InstallationTargetType {
    fn to_string(&self) -> String {
        match *self {
            InstallationTargetType::User => "User".to_string(),
            InstallationTargetType::Organization => "Organization".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InstallationUpdatedAt {
    Variant0(chrono::DateTime<chrono::offset::Utc>),
    Variant1(i64),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for InstallationCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationCreatedRepositoriesItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for InstallationDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationDeletedRepositoriesItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationNewPermissionsAcceptedAction {
    #[serde(rename = "new_permissions_accepted")]
    NewPermissionsAccepted,
}
impl ToString for InstallationNewPermissionsAcceptedAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationNewPermissionsAcceptedAction::NewPermissionsAccepted => {
                "new_permissions_accepted".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationNewPermissionsAcceptedRepositoriesItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationSuspendAction {
    #[serde(rename = "suspend")]
    Suspend,
}
impl ToString for InstallationSuspendAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationSuspendAction::Suspend => "suspend".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationSuspendRepositoriesItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationUnsuspendAction {
    #[serde(rename = "unsuspend")]
    Unsuspend,
}
impl ToString for InstallationUnsuspendAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationUnsuspendAction::Unsuspend => "unsuspend".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationUnsuspendRepositoriesItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationRepositoriesAddedAction {
    #[serde(rename = "added")]
    Added,
}
impl ToString for InstallationRepositoriesAddedAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationRepositoriesAddedAction::Added => "added".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesAddedRepositoriesAddedItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesAddedRepositoriesRemovedItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "Unique identifier of the repository"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "Whether the repository is private or public."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}
#[doc = "Describe whether all repositories have been selected or there's a selection involved"]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationRepositoriesAddedRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
}
impl ToString for InstallationRepositoriesAddedRepositorySelection {
    fn to_string(&self) -> String {
        match *self {
            InstallationRepositoriesAddedRepositorySelection::All => "all".to_string(),
            InstallationRepositoriesAddedRepositorySelection::Selected => "selected".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationRepositoriesRemovedAction {
    #[serde(rename = "removed")]
    Removed,
}
impl ToString for InstallationRepositoriesRemovedAction {
    fn to_string(&self) -> String {
        match *self {
            InstallationRepositoriesRemovedAction::Removed => "removed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesRemovedRepositoriesAddedItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InstallationRepositoriesRemovedRepositoriesRemovedItem {
    pub full_name: String,
    #[doc = "Unique identifier of the repository"]
    pub id: i64,
    #[doc = "The name of the repository."]
    pub name: String,
    pub node_id: String,
    #[doc = "Whether the repository is private or public."]
    pub private: bool,
}
#[doc = "Describe whether all repositories have been selected or there's a selection involved"]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum InstallationRepositoriesRemovedRepositorySelection {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "selected")]
    Selected,
}
impl ToString for InstallationRepositoriesRemovedRepositorySelection {
    fn to_string(&self) -> String {
        match *self {
            InstallationRepositoriesRemovedRepositorySelection::All => "all".to_string(),
            InstallationRepositoriesRemovedRepositorySelection::Selected => "selected".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssueActiveLockReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "spam")]
    Spam,
}
impl ToString for IssueActiveLockReason {
    fn to_string(&self) -> String {
        match *self {
            IssueActiveLockReason::Resolved => "resolved".to_string(),
            IssueActiveLockReason::OffTopic => "off-topic".to_string(),
            IssueActiveLockReason::TooHeated => "too heated".to_string(),
            IssueActiveLockReason::Spam => "spam".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuePullRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diff_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[doc = "State of the issue; either 'open' or 'closed'"]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssueState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for IssueState {
    fn to_string(&self) -> String {
        match *self {
            IssueState::Open => "open".to_string(),
            IssueState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssueCommentCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for IssueCommentCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            IssueCommentCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/issues#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentCreatedComment {
    pub author_association: AuthorAssociation,
    #[doc = "Contents of the issue comment"]
    pub body: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub html_url: String,
    #[doc = "Unique identifier of the issue comment"]
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub performed_via_github_app: Option<App>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the issue comment"]
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssueCommentDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for IssueCommentDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            IssueCommentDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/issues#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentDeletedComment {
    pub author_association: AuthorAssociation,
    #[doc = "Contents of the issue comment"]
    pub body: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub html_url: String,
    #[doc = "Unique identifier of the issue comment"]
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the issue comment"]
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssueCommentEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for IssueCommentEditedAction {
    fn to_string(&self) -> String {
        match *self {
            IssueCommentEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentEditedChangesBody {
    #[doc = "The previous version of the body."]
    pub from: String,
}
#[doc = "The changes to the comment."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssueCommentEditedChangesBody>,
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/issues#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssueCommentEditedComment {
    pub author_association: AuthorAssociation,
    #[doc = "Contents of the issue comment"]
    pub body: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    pub html_url: String,
    #[doc = "Unique identifier of the issue comment"]
    pub id: i64,
    pub issue_url: String,
    pub node_id: String,
    pub performed_via_github_app: Option<App>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the issue comment"]
    pub url: String,
    pub user: User,
}
#[doc = "The action that was performed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesAssignedAction {
    #[serde(rename = "assigned")]
    Assigned,
}
impl ToString for IssuesAssignedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesAssignedAction::Assigned => "assigned".to_string(),
        }
    }
}
#[doc = "The action that was performed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesClosedAction {
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for IssuesClosedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesClosedAction::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for IssuesDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesDemilestonedAction {
    #[serde(rename = "demilestoned")]
    Demilestoned,
}
impl ToString for IssuesDemilestonedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesDemilestonedAction::Demilestoned => "demilestoned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for IssuesEditedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesEditedChangesBody {
    #[doc = "The previous version of the body."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesEditedChangesTitle {
    #[doc = "The previous version of the title."]
    pub from: String,
}
#[doc = "The changes to the issue."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct IssuesEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<IssuesEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<IssuesEditedChangesTitle>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesLabeledAction {
    #[serde(rename = "labeled")]
    Labeled,
}
impl ToString for IssuesLabeledAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesLabeledAction::Labeled => "labeled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesLockedAction {
    #[serde(rename = "locked")]
    Locked,
}
impl ToString for IssuesLockedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesLockedAction::Locked => "locked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesMilestonedAction {
    #[serde(rename = "milestoned")]
    Milestoned,
}
impl ToString for IssuesMilestonedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesMilestonedAction::Milestoned => "milestoned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesOpenedAction {
    #[serde(rename = "opened")]
    Opened,
}
impl ToString for IssuesOpenedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesOpenedAction::Opened => "opened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesPinnedAction {
    #[serde(rename = "pinned")]
    Pinned,
}
impl ToString for IssuesPinnedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesPinnedAction::Pinned => "pinned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesReopenedAction {
    #[serde(rename = "reopened")]
    Reopened,
}
impl ToString for IssuesReopenedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesReopenedAction::Reopened => "reopened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesTransferredAction {
    #[serde(rename = "transferred")]
    Transferred,
}
impl ToString for IssuesTransferredAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesTransferredAction::Transferred => "transferred".to_string(),
        }
    }
}
#[doc = "The action that was performed."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesUnassignedAction {
    #[serde(rename = "unassigned")]
    Unassigned,
}
impl ToString for IssuesUnassignedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesUnassignedAction::Unassigned => "unassigned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesUnlabeledAction {
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ToString for IssuesUnlabeledAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesUnlabeledAction::Unlabeled => "unlabeled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesUnlockedAction {
    #[serde(rename = "unlocked")]
    Unlocked,
}
impl ToString for IssuesUnlockedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesUnlockedAction::Unlocked => "unlocked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum IssuesUnpinnedAction {
    #[serde(rename = "unpinned")]
    Unpinned,
}
impl ToString for IssuesUnpinnedAction {
    fn to_string(&self) -> String {
        match *self {
            IssuesUnpinnedAction::Unpinned => "unpinned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum LabelCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for LabelCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            LabelCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum LabelDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for LabelDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            LabelDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum LabelEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for LabelEditedAction {
    fn to_string(&self) -> String {
        match *self {
            LabelEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelEditedChangesColor {
    #[doc = "The previous version of the color if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelEditedChangesName {
    #[doc = "The previous version of the name if the action was `edited`."]
    pub from: String,
}
#[doc = "The changes to the label if the action was `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct LabelEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<LabelEditedChangesColor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<LabelEditedChangesName>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchaseAccount {
    pub id: i64,
    pub login: String,
    pub organization_billing_email: String,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePlan {
    pub bullets: Vec<String>,
    pub description: String,
    pub has_free_trial: bool,
    pub id: i64,
    pub monthly_price_in_cents: i64,
    pub name: String,
    pub price_model: String,
    pub unit_name: Option<String>,
    pub yearly_price_in_cents: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MarketplacePurchaseCancelledAction {
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ToString for MarketplacePurchaseCancelledAction {
    fn to_string(&self) -> String {
        match *self {
            MarketplacePurchaseCancelledAction::Cancelled => "cancelled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchaseCancelledSender {
    pub avatar_url: String,
    pub email: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MarketplacePurchaseChangedAction {
    #[serde(rename = "changed")]
    Changed,
}
impl ToString for MarketplacePurchaseChangedAction {
    fn to_string(&self) -> String {
        match *self {
            MarketplacePurchaseChangedAction::Changed => "changed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchaseChangedSender {
    pub avatar_url: String,
    pub email: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MarketplacePurchasePendingChangeAction {
    #[serde(rename = "pending_change")]
    PendingChange,
}
impl ToString for MarketplacePurchasePendingChangeAction {
    fn to_string(&self) -> String {
        match *self {
            MarketplacePurchasePendingChangeAction::PendingChange => "pending_change".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePendingChangeSender {
    pub avatar_url: String,
    pub email: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MarketplacePurchasePendingChangeCancelledAction {
    #[serde(rename = "pending_change_cancelled")]
    PendingChangeCancelled,
}
impl ToString for MarketplacePurchasePendingChangeCancelledAction {
    fn to_string(&self) -> String {
        match *self {
            MarketplacePurchasePendingChangeCancelledAction::PendingChangeCancelled => {
                "pending_change_cancelled".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePendingChangeCancelledSender {
    pub avatar_url: String,
    pub email: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MarketplacePurchasePurchasedAction {
    #[serde(rename = "purchased")]
    Purchased,
}
impl ToString for MarketplacePurchasePurchasedAction {
    fn to_string(&self) -> String {
        match *self {
            MarketplacePurchasePurchasedAction::Purchased => "purchased".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MarketplacePurchasePurchasedSender {
    pub avatar_url: String,
    pub email: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MemberAddedAction {
    #[serde(rename = "added")]
    Added,
}
impl ToString for MemberAddedAction {
    fn to_string(&self) -> String {
        match *self {
            MemberAddedAction::Added => "added".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MemberEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for MemberEditedAction {
    fn to_string(&self) -> String {
        match *self {
            MemberEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MemberEditedChangesOldPermission {
    #[doc = "The previous permissions of the collaborator if the action was edited."]
    pub from: String,
}
#[doc = "The changes to the collaborator permissions"]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MemberEditedChanges {
    pub old_permission: MemberEditedChangesOldPermission,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MemberRemovedAction {
    #[serde(rename = "removed")]
    Removed,
}
impl ToString for MemberRemovedAction {
    fn to_string(&self) -> String {
        match *self {
            MemberRemovedAction::Removed => "removed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MembershipAddedAction {
    #[serde(rename = "added")]
    Added,
}
impl ToString for MembershipAddedAction {
    fn to_string(&self) -> String {
        match *self {
            MembershipAddedAction::Added => "added".to_string(),
        }
    }
}
#[doc = "The scope of the membership. Currently, can only be `team`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MembershipAddedScope {
    #[serde(rename = "team")]
    Team,
}
impl ToString for MembershipAddedScope {
    fn to_string(&self) -> String {
        match *self {
            MembershipAddedScope::Team => "team".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MembershipRemovedAction {
    #[serde(rename = "removed")]
    Removed,
}
impl ToString for MembershipRemovedAction {
    fn to_string(&self) -> String {
        match *self {
            MembershipRemovedAction::Removed => "removed".to_string(),
        }
    }
}
#[doc = "The scope of the membership. Currently, can only be `team`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MembershipRemovedScope {
    #[serde(rename = "team")]
    Team,
}
impl ToString for MembershipRemovedScope {
    fn to_string(&self) -> String {
        match *self {
            MembershipRemovedScope::Team => "team".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MetaDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for MetaDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            MetaDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetaDeletedHookConfig {
    pub content_type: String,
    pub insecure_ssl: String,
    pub url: String,
}
#[doc = "The modified webhook. This will contain different keys based on the type of webhook it is: repository, organization, business, app, or GitHub Marketplace."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MetaDeletedHook {
    pub active: bool,
    pub config: MetaDeletedHookConfig,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub updated_at: String,
}
#[doc = "The state of the milestone."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for MilestoneState {
    fn to_string(&self) -> String {
        match *self {
            MilestoneState::Open => "open".to_string(),
            MilestoneState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneClosedAction {
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for MilestoneClosedAction {
    fn to_string(&self) -> String {
        match *self {
            MilestoneClosedAction::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for MilestoneCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            MilestoneCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for MilestoneDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            MilestoneDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for MilestoneEditedAction {
    fn to_string(&self) -> String {
        match *self {
            MilestoneEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneEditedChangesDescription {
    #[doc = "The previous version of the description if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneEditedChangesDueOn {
    #[doc = "The previous version of the due date if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneEditedChangesTitle {
    #[doc = "The previous version of the title if the action was `edited`."]
    pub from: String,
}
#[doc = "The changes to the milestone if the action was `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct MilestoneEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<MilestoneEditedChangesDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_on: Option<MilestoneEditedChangesDueOn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<MilestoneEditedChangesTitle>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum MilestoneOpenedAction {
    #[serde(rename = "opened")]
    Opened,
}
impl ToString for MilestoneOpenedAction {
    fn to_string(&self) -> String {
        match *self {
            MilestoneOpenedAction::Opened => "opened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrgBlockBlockedAction {
    #[serde(rename = "blocked")]
    Blocked,
}
impl ToString for OrgBlockBlockedAction {
    fn to_string(&self) -> String {
        match *self {
            OrgBlockBlockedAction::Blocked => "blocked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrgBlockUnblockedAction {
    #[serde(rename = "unblocked")]
    Unblocked,
}
impl ToString for OrgBlockUnblockedAction {
    fn to_string(&self) -> String {
        match *self {
            OrgBlockUnblockedAction::Unblocked => "unblocked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrganizationDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for OrganizationDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            OrganizationDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrganizationMemberAddedAction {
    #[serde(rename = "member_added")]
    MemberAdded,
}
impl ToString for OrganizationMemberAddedAction {
    fn to_string(&self) -> String {
        match *self {
            OrganizationMemberAddedAction::MemberAdded => "member_added".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrganizationMemberInvitedAction {
    #[serde(rename = "member_invited")]
    MemberInvited,
}
impl ToString for OrganizationMemberInvitedAction {
    fn to_string(&self) -> String {
        match *self {
            OrganizationMemberInvitedAction::MemberInvited => "member_invited".to_string(),
        }
    }
}
#[doc = "The invitation for the user or email if the action is `member_invited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OrganizationMemberInvitedInvitation {}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrganizationMemberRemovedAction {
    #[serde(rename = "member_removed")]
    MemberRemoved,
}
impl ToString for OrganizationMemberRemovedAction {
    fn to_string(&self) -> String {
        match *self {
            OrganizationMemberRemovedAction::MemberRemoved => "member_removed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum OrganizationRenamedAction {
    #[serde(rename = "renamed")]
    Renamed,
}
impl ToString for OrganizationRenamedAction {
    fn to_string(&self) -> String {
        match *self {
            OrganizationRenamedAction::Renamed => "renamed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PackagePublishedAction {
    #[serde(rename = "published")]
    Published,
}
impl ToString for PackagePublishedAction {
    fn to_string(&self) -> String {
        match *self {
            PackagePublishedAction::Published => "published".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublishedPackagePackageVersionPackageFilesItem {
    pub content_type: String,
    pub created_at: String,
    pub download_url: String,
    pub id: i64,
    pub md5: String,
    pub name: String,
    pub sha1: String,
    pub sha256: String,
    pub size: i64,
    pub state: String,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublishedPackagePackageVersionRelease {
    pub author: User,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublishedPackagePackageVersion {
    pub author: User,
    pub body: String,
    pub body_html: String,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    pub manifest: String,
    pub metadata: Vec<serde_json::Value>,
    pub package_files: Vec<PackagePublishedPackagePackageVersionPackageFilesItem>,
    pub prerelease: bool,
    pub release: PackagePublishedPackagePackageVersionRelease,
    pub summary: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub target_oid: String,
    pub updated_at: String,
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublishedPackageRegistry {
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub vendor: String,
}
#[doc = "Information about the package."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackagePublishedPackage {
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub owner: User,
    pub package_type: String,
    pub package_version: PackagePublishedPackagePackageVersion,
    pub registry: PackagePublishedPackageRegistry,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PackageUpdatedAction {
    #[serde(rename = "updated")]
    Updated,
}
impl ToString for PackageUpdatedAction {
    fn to_string(&self) -> String {
        match *self {
            PackageUpdatedAction::Updated => "updated".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdatedPackagePackageVersionPackageFilesItem {
    pub content_type: String,
    pub created_at: String,
    pub download_url: String,
    pub id: i64,
    pub md5: String,
    pub name: String,
    pub sha1: String,
    pub sha256: String,
    pub size: i64,
    pub state: String,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdatedPackagePackageVersionRelease {
    pub author: User,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdatedPackagePackageVersion {
    pub author: User,
    pub body: String,
    pub body_html: String,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub installation_command: String,
    pub manifest: String,
    pub metadata: Vec<serde_json::Value>,
    pub package_files: Vec<PackageUpdatedPackagePackageVersionPackageFilesItem>,
    pub prerelease: bool,
    pub release: PackageUpdatedPackagePackageVersionRelease,
    pub summary: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub target_oid: String,
    pub updated_at: String,
    pub version: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdatedPackageRegistry {
    pub about_url: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub vendor: String,
}
#[doc = "Information about the package."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PackageUpdatedPackage {
    pub created_at: String,
    pub html_url: String,
    pub id: i64,
    pub name: String,
    pub owner: User,
    pub package_type: String,
    pub package_version: PackageUpdatedPackagePackageVersion,
    pub registry: PackageUpdatedPackageRegistry,
    pub updated_at: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PageBuildEventBuildError {
    pub message: Option<String>,
}
#[doc = "The [List GitHub Pages builds](https://docs.github.com/en/rest/reference/repos#list-github-pages-builds) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PageBuildEventBuild {
    pub commit: String,
    pub created_at: String,
    pub duration: i64,
    pub error: PageBuildEventBuildError,
    pub pusher: User,
    pub status: String,
    pub updated_at: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PingEventHookConfig {
    pub content_type: String,
    pub insecure_ssl: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PingEventHookLastResponse {
    pub code: (),
    pub message: (),
    pub status: String,
}
#[doc = "The [webhook configuration](https://docs.github.com/en/rest/reference/repos#get-a-repository-webhook)."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
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
#[doc = "State of the project; either 'open' or 'closed'"]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for ProjectState {
    fn to_string(&self) -> String {
        match *self {
            ProjectState::Open => "open".to_string(),
            ProjectState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectClosedAction {
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for ProjectClosedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectClosedAction::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for ProjectCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for ProjectDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for ProjectEditedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectEditedChangesBody {
    #[doc = "The previous version of the body if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectEditedChangesName {
    #[doc = "The changes to the project if the action was `edited`."]
    pub from: String,
}
#[doc = "The changes to the project if the action was `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<ProjectEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ProjectEditedChangesName>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectReopenedAction {
    #[serde(rename = "reopened")]
    Reopened,
}
impl ToString for ProjectReopenedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectReopenedAction::Reopened => "reopened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCardConvertedAction {
    #[serde(rename = "converted")]
    Converted,
}
impl ToString for ProjectCardConvertedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCardConvertedAction::Converted => "converted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardConvertedChangesNote {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardConvertedChanges {
    pub note: ProjectCardConvertedChangesNote,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCardCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for ProjectCardCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCardCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCardDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for ProjectCardDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCardDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCardEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for ProjectCardEditedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCardEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardEditedChangesNote {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardEditedChanges {
    pub note: ProjectCardEditedChangesNote,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectCardMovedAction {
    #[serde(rename = "moved")]
    Moved,
}
impl ToString for ProjectCardMovedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectCardMovedAction::Moved => "moved".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardMovedChangesColumnId {
    pub from: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ProjectCardMovedChanges {
    pub column_id: ProjectCardMovedChangesColumnId,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectColumnCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for ProjectColumnCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectColumnCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectColumnDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for ProjectColumnDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectColumnDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectColumnEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for ProjectColumnEditedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectColumnEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ProjectColumnMovedAction {
    #[serde(rename = "moved")]
    Moved,
}
impl ToString for ProjectColumnMovedAction {
    fn to_string(&self) -> String {
        match *self {
            ProjectColumnMovedAction::Moved => "moved".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestActiveLockReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "spam")]
    Spam,
}
impl ToString for PullRequestActiveLockReason {
    fn to_string(&self) -> String {
        match *self {
            PullRequestActiveLockReason::Resolved => "resolved".to_string(),
            PullRequestActiveLockReason::OffTopic => "off-topic".to_string(),
            PullRequestActiveLockReason::TooHeated => "too heated".to_string(),
            PullRequestActiveLockReason::Spam => "spam".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PullRequestRequestedReviewersItem {
    User(User),
    Team(Team),
}
#[doc = "State of this Pull Request. Either `open` or `closed`."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for PullRequestState {
    fn to_string(&self) -> String {
        match *self {
            PullRequestState::Open => "open".to_string(),
            PullRequestState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestAssignedAction {
    #[serde(rename = "assigned")]
    Assigned,
}
impl ToString for PullRequestAssignedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestAssignedAction::Assigned => "assigned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestAutoMergeDisabledAction {
    #[serde(rename = "auto_merge_disabled")]
    AutoMergeDisabled,
}
impl ToString for PullRequestAutoMergeDisabledAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestAutoMergeDisabledAction::AutoMergeDisabled => {
                "auto_merge_disabled".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestAutoMergeEnabledAction {
    #[serde(rename = "auto_merge_enabled")]
    AutoMergeEnabled,
}
impl ToString for PullRequestAutoMergeEnabledAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestAutoMergeEnabledAction::AutoMergeEnabled => "auto_merge_enabled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestClosedAction {
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for PullRequestClosedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestClosedAction::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestConvertedToDraftAction {
    #[serde(rename = "converted_to_draft")]
    ConvertedToDraft,
}
impl ToString for PullRequestConvertedToDraftAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestConvertedToDraftAction::ConvertedToDraft => "converted_to_draft".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for PullRequestEditedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestEditedChangesBody {
    #[doc = "The previous version of the body if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestEditedChangesTitle {
    #[doc = "The previous version of the title if the action was `edited`."]
    pub from: String,
}
#[doc = "The changes to the comment if the action was `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<PullRequestEditedChangesTitle>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestLabeledAction {
    #[serde(rename = "labeled")]
    Labeled,
}
impl ToString for PullRequestLabeledAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestLabeledAction::Labeled => "labeled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestLockedAction {
    #[serde(rename = "locked")]
    Locked,
}
impl ToString for PullRequestLockedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestLockedAction::Locked => "locked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestOpenedAction {
    #[serde(rename = "opened")]
    Opened,
}
impl ToString for PullRequestOpenedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestOpenedAction::Opened => "opened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReadyForReviewAction {
    #[serde(rename = "ready_for_review")]
    ReadyForReview,
}
impl ToString for PullRequestReadyForReviewAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReadyForReviewAction::ReadyForReview => "ready_for_review".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReopenedAction {
    #[serde(rename = "reopened")]
    Reopened,
}
impl ToString for PullRequestReopenedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReopenedAction::Reopened => "reopened".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewRequestRemovedAction {
    #[serde(rename = "review_request_removed")]
    ReviewRequestRemoved,
}
impl ToString for PullRequestReviewRequestRemovedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewRequestRemovedAction::ReviewRequestRemoved => {
                "review_request_removed".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewRequestedAction {
    #[serde(rename = "review_requested")]
    ReviewRequested,
}
impl ToString for PullRequestReviewRequestedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewRequestedAction::ReviewRequested => "review_requested".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestSynchronizeAction {
    #[serde(rename = "synchronize")]
    Synchronize,
}
impl ToString for PullRequestSynchronizeAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestSynchronizeAction::Synchronize => "synchronize".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestUnassignedAction {
    #[serde(rename = "unassigned")]
    Unassigned,
}
impl ToString for PullRequestUnassignedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestUnassignedAction::Unassigned => "unassigned".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestUnlabeledAction {
    #[serde(rename = "unlabeled")]
    Unlabeled,
}
impl ToString for PullRequestUnlabeledAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestUnlabeledAction::Unlabeled => "unlabeled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestUnlockedAction {
    #[serde(rename = "unlocked")]
    Unlocked,
}
impl ToString for PullRequestUnlockedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestUnlockedAction::Unlocked => "unlocked".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewDismissedAction {
    #[serde(rename = "dismissed")]
    Dismissed,
}
impl ToString for PullRequestReviewDismissedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewDismissedAction::Dismissed => "dismissed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewDismissedReviewLinks {
    pub html: Link,
    pub pull_request: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewDismissedReviewState {
    #[serde(rename = "dismissed")]
    Dismissed,
}
impl ToString for PullRequestReviewDismissedReviewState {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewDismissedReviewState::Dismissed => "dismissed".to_string(),
        }
    }
}
#[doc = "The review that was affected."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewDismissedReview {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the review."]
    pub body: Option<String>,
    #[doc = "A commit SHA for the review."]
    pub commit_id: String,
    pub html_url: String,
    #[doc = "Unique identifier of the review"]
    pub id: i64,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewDismissedReviewLinks,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: PullRequestReviewDismissedReviewState,
    pub submitted_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for PullRequestReviewEditedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewEditedChangesBody {
    #[doc = "The previous version of the body if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewEditedChanges {
    pub body: PullRequestReviewEditedChangesBody,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewEditedReviewLinks {
    pub html: Link,
    pub pull_request: Link,
}
#[doc = "The review that was affected."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewEditedReview {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the review."]
    pub body: Option<String>,
    #[doc = "A commit SHA for the review."]
    pub commit_id: String,
    pub html_url: String,
    #[doc = "Unique identifier of the review"]
    pub id: i64,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewEditedReviewLinks,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: String,
    pub submitted_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewSubmittedAction {
    #[serde(rename = "submitted")]
    Submitted,
}
impl ToString for PullRequestReviewSubmittedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewSubmittedAction::Submitted => "submitted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewSubmittedReviewLinks {
    pub html: Link,
    pub pull_request: Link,
}
#[doc = "The review that was affected."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewSubmittedReview {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the review."]
    pub body: Option<String>,
    #[doc = "A commit SHA for the review."]
    pub commit_id: String,
    pub html_url: String,
    #[doc = "Unique identifier of the review"]
    pub id: i64,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewSubmittedReviewLinks,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: String,
    pub submitted_at: chrono::DateTime<chrono::offset::Utc>,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for PullRequestReviewCommentCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedCommentLinks {
    pub html: Link,
    pub pull_request: Link,
    #[serde(rename = "self")]
    pub self_: Link,
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentCreatedCommentSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentCreatedCommentSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentCreatedCommentSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentCreatedCommentSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentCreatedCommentStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentCreatedCommentStartSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentCreatedCommentStartSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentCreatedCommentStartSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/pulls#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedComment {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the comment."]
    pub body: String,
    #[doc = "The SHA of the commit to which the comment applies."]
    pub commit_id: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The diff of the line that the comment refers to."]
    pub diff_hunk: String,
    #[doc = "HTML URL for the pull request review comment."]
    pub html_url: String,
    #[doc = "The ID of the pull request review comment."]
    pub id: i64,
    #[doc = "The comment ID to reply to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedCommentLinks,
    #[doc = "The node ID of the pull request review comment."]
    pub node_id: String,
    #[doc = "The SHA of the original commit to which the comment applies."]
    pub original_commit_id: String,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i64>,
    #[doc = "The index of the original line in the diff to which the comment applies."]
    pub original_position: i64,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i64>,
    #[doc = "The relative path of the file to which the comment applies."]
    pub path: String,
    #[doc = "The line index in the diff to which the comment applies."]
    pub position: i64,
    #[doc = "The ID of the pull request review to which the comment belongs."]
    pub pull_request_review_id: i64,
    #[doc = "URL for the pull request that the review comment belongs to."]
    pub pull_request_url: String,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PullRequestReviewCommentCreatedCommentSide>,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default = "pull_request_review_comment_created_comment_start_side")]
    pub start_side: Option<PullRequestReviewCommentCreatedCommentStartSide>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the pull request review comment"]
    pub url: String,
    pub user: User,
}
fn pull_request_review_comment_created_comment_start_side(
) -> Option<PullRequestReviewCommentCreatedCommentStartSide> {
    Some(PullRequestReviewCommentCreatedCommentStartSide::Right)
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedPullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedPullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PullRequestReviewCommentCreatedPullRequestRequestedReviewersItem {
    User(User),
    Team(Team),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentCreatedPullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for PullRequestReviewCommentCreatedPullRequestState {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentCreatedPullRequestState::Open => "open".to_string(),
            PullRequestReviewCommentCreatedPullRequestState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentCreatedPullRequest {
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    pub base: PullRequestReviewCommentCreatedPullRequestBase,
    pub body: String,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullRequestReviewCommentCreatedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<Label>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentCreatedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<PullRequestReviewCommentCreatedPullRequestRequestedReviewersItem>,
    pub requested_teams: Vec<Team>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: PullRequestReviewCommentCreatedPullRequestState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for PullRequestReviewCommentDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedCommentLinks {
    pub html: Link,
    pub pull_request: Link,
    #[serde(rename = "self")]
    pub self_: Link,
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentDeletedCommentSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentDeletedCommentSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentDeletedCommentSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentDeletedCommentSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentDeletedCommentStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentDeletedCommentStartSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentDeletedCommentStartSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentDeletedCommentStartSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/pulls#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedComment {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the comment."]
    pub body: String,
    #[doc = "The SHA of the commit to which the comment applies."]
    pub commit_id: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The diff of the line that the comment refers to."]
    pub diff_hunk: String,
    #[doc = "HTML URL for the pull request review comment."]
    pub html_url: String,
    #[doc = "The ID of the pull request review comment."]
    pub id: i64,
    #[doc = "The comment ID to reply to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentDeletedCommentLinks,
    #[doc = "The node ID of the pull request review comment."]
    pub node_id: String,
    #[doc = "The SHA of the original commit to which the comment applies."]
    pub original_commit_id: String,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i64>,
    #[doc = "The index of the original line in the diff to which the comment applies."]
    pub original_position: i64,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i64>,
    #[doc = "The relative path of the file to which the comment applies."]
    pub path: String,
    #[doc = "The line index in the diff to which the comment applies."]
    pub position: i64,
    #[doc = "The ID of the pull request review to which the comment belongs."]
    pub pull_request_review_id: i64,
    #[doc = "URL for the pull request that the review comment belongs to."]
    pub pull_request_url: String,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PullRequestReviewCommentDeletedCommentSide>,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default = "pull_request_review_comment_deleted_comment_start_side")]
    pub start_side: Option<PullRequestReviewCommentDeletedCommentStartSide>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the pull request review comment"]
    pub url: String,
    pub user: User,
}
fn pull_request_review_comment_deleted_comment_start_side(
) -> Option<PullRequestReviewCommentDeletedCommentStartSide> {
    Some(PullRequestReviewCommentDeletedCommentStartSide::Right)
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedPullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedPullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PullRequestReviewCommentDeletedPullRequestRequestedReviewersItem {
    User(User),
    Team(Team),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentDeletedPullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for PullRequestReviewCommentDeletedPullRequestState {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentDeletedPullRequestState::Open => "open".to_string(),
            PullRequestReviewCommentDeletedPullRequestState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentDeletedPullRequest {
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    pub base: PullRequestReviewCommentDeletedPullRequestBase,
    pub body: String,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullRequestReviewCommentDeletedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<Label>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentDeletedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<PullRequestReviewCommentDeletedPullRequestRequestedReviewersItem>,
    pub requested_teams: Vec<Team>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: PullRequestReviewCommentDeletedPullRequestState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for PullRequestReviewCommentEditedAction {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedChangesBody {
    #[doc = "The previous version of the body."]
    pub from: String,
}
#[doc = "The changes to the comment."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PullRequestReviewCommentEditedChangesBody>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedCommentLinks {
    pub html: Link,
    pub pull_request: Link,
    #[serde(rename = "self")]
    pub self_: Link,
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentEditedCommentSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentEditedCommentSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentEditedCommentSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentEditedCommentSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The side of the first line of the range for a multi-line comment."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentEditedCommentStartSide {
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
}
impl ToString for PullRequestReviewCommentEditedCommentStartSide {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentEditedCommentStartSide::Left => "LEFT".to_string(),
            PullRequestReviewCommentEditedCommentStartSide::Right => "RIGHT".to_string(),
        }
    }
}
#[doc = "The [comment](https://docs.github.com/en/rest/reference/pulls#comments) itself."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedComment {
    pub author_association: AuthorAssociation,
    #[doc = "The text of the comment."]
    pub body: String,
    #[doc = "The SHA of the commit to which the comment applies."]
    pub commit_id: String,
    pub created_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "The diff of the line that the comment refers to."]
    pub diff_hunk: String,
    #[doc = "HTML URL for the pull request review comment."]
    pub html_url: String,
    #[doc = "The ID of the pull request review comment."]
    pub id: i64,
    #[doc = "The comment ID to reply to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to_id: Option<i64>,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentEditedCommentLinks,
    #[doc = "The node ID of the pull request review comment."]
    pub node_id: String,
    #[doc = "The SHA of the original commit to which the comment applies."]
    pub original_commit_id: String,
    #[doc = "The line of the blob to which the comment applies. The last line of the range for a multi-line comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_line: Option<i64>,
    #[doc = "The index of the original line in the diff to which the comment applies."]
    pub original_position: i64,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_line: Option<i64>,
    #[doc = "The relative path of the file to which the comment applies."]
    pub path: String,
    #[doc = "The line index in the diff to which the comment applies."]
    pub position: i64,
    #[doc = "The ID of the pull request review to which the comment belongs."]
    pub pull_request_review_id: i64,
    #[doc = "URL for the pull request that the review comment belongs to."]
    pub pull_request_url: String,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<PullRequestReviewCommentEditedCommentSide>,
    #[doc = "The first line of the range for a multi-line comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i64>,
    #[doc = "The side of the first line of the range for a multi-line comment."]
    #[serde(default = "pull_request_review_comment_edited_comment_start_side")]
    pub start_side: Option<PullRequestReviewCommentEditedCommentStartSide>,
    pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    #[doc = "URL for the pull request review comment"]
    pub url: String,
    pub user: User,
}
fn pull_request_review_comment_edited_comment_start_side(
) -> Option<PullRequestReviewCommentEditedCommentStartSide> {
    Some(PullRequestReviewCommentEditedCommentStartSide::Right)
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedPullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedPullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedPullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PullRequestReviewCommentEditedPullRequestRequestedReviewersItem {
    User(User),
    Team(Team),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum PullRequestReviewCommentEditedPullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for PullRequestReviewCommentEditedPullRequestState {
    fn to_string(&self) -> String {
        match *self {
            PullRequestReviewCommentEditedPullRequestState::Open => "open".to_string(),
            PullRequestReviewCommentEditedPullRequestState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewCommentEditedPullRequest {
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    pub author_association: AuthorAssociation,
    pub base: PullRequestReviewCommentEditedPullRequestBase,
    pub body: String,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullRequestReviewCommentEditedPullRequestHead,
    pub html_url: String,
    pub id: i64,
    pub issue_url: String,
    pub labels: Vec<Label>,
    #[serde(rename = "_links")]
    pub links: PullRequestReviewCommentEditedPullRequestLinks,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<Milestone>,
    pub node_id: String,
    pub number: i64,
    pub patch_url: String,
    pub requested_reviewers: Vec<PullRequestReviewCommentEditedPullRequestRequestedReviewersItem>,
    pub requested_teams: Vec<Team>,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: PullRequestReviewCommentEditedPullRequestState,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for ReleaseCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleaseCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseCreatedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for ReleaseDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleaseDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseDeletedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for ReleaseEditedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleaseEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseEditedChangesBody {
    #[doc = "The previous version of the body if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseEditedChangesName {
    #[doc = "The previous version of the name if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<ReleaseEditedChangesBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ReleaseEditedChangesName>,
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseEditedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleasePrereleasedAction {
    #[serde(rename = "prereleased")]
    Prereleased,
}
impl ToString for ReleasePrereleasedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleasePrereleasedAction::Prereleased => "prereleased".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleasePrereleasedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleasePublishedAction {
    #[serde(rename = "published")]
    Published,
}
impl ToString for ReleasePublishedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleasePublishedAction::Published => "published".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleasePublishedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseReleasedAction {
    #[serde(rename = "released")]
    Released,
}
impl ToString for ReleaseReleasedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleaseReleasedAction::Released => "released".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseReleasedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseUnpublishedAction {
    #[serde(rename = "unpublished")]
    Unpublished,
}
impl ToString for ReleaseUnpublishedAction {
    fn to_string(&self) -> String {
        match *self {
            ReleaseUnpublishedAction::Unpublished => "unpublished".to_string(),
        }
    }
}
#[doc = "The [release](https://docs.github.com/en/rest/reference/repos/#get-a-release) object."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ReleaseUnpublishedRelease {
    pub assets: Vec<ReleaseAsset>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "true to create a draft (unpublished) release, false to create a published one."]
    pub draft: bool,
    pub html_url: String,
    pub id: i64,
    pub name: (),
    pub node_id: String,
    #[doc = "Whether the release is identified as a prerelease or a full release."]
    pub prerelease: bool,
    pub published_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "The name of the tag."]
    pub tag_name: String,
    pub tarball_url: Option<String>,
    #[doc = "Specifies the commitish value that determines where the Git tag is created from."]
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: Option<String>,
}
#[doc = "State of the release asset."]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ReleaseAssetState {
    #[serde(rename = "uploaded")]
    Uploaded,
}
impl ToString for ReleaseAssetState {
    fn to_string(&self) -> String {
        match *self {
            ReleaseAssetState::Uploaded => "uploaded".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RepositoryCreatedAt {
    Variant0(i64),
    Variant1(chrono::DateTime<chrono::offset::Utc>),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryPermissions {
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    pub pull: bool,
    pub push: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RepositoryPushedAt {
    Variant0(i64),
    Variant1(chrono::DateTime<chrono::offset::Utc>),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryArchivedAction {
    #[serde(rename = "archived")]
    Archived,
}
impl ToString for RepositoryArchivedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryArchivedAction::Archived => "archived".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for RepositoryCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for RepositoryDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for RepositoryEditedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEditedChangesDefaultBranch {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEditedChangesDescription {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEditedChangesHomepage {
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<RepositoryEditedChangesDefaultBranch>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<RepositoryEditedChangesDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<RepositoryEditedChangesHomepage>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryPrivatizedAction {
    #[serde(rename = "privatized")]
    Privatized,
}
impl ToString for RepositoryPrivatizedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryPrivatizedAction::Privatized => "privatized".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryPublicizedAction {
    #[serde(rename = "publicized")]
    Publicized,
}
impl ToString for RepositoryPublicizedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryPublicizedAction::Publicized => "publicized".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryRenamedAction {
    #[serde(rename = "renamed")]
    Renamed,
}
impl ToString for RepositoryRenamedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryRenamedAction::Renamed => "renamed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryTransferredAction {
    #[serde(rename = "transferred")]
    Transferred,
}
impl ToString for RepositoryTransferredAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryTransferredAction::Transferred => "transferred".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryTransferredChangesOwnerFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryTransferredChangesOwner {
    pub from: RepositoryTransferredChangesOwnerFrom,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryTransferredChanges {
    pub owner: RepositoryTransferredChangesOwner,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryUnarchivedAction {
    #[serde(rename = "unarchived")]
    Unarchived,
}
impl ToString for RepositoryUnarchivedAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryUnarchivedAction::Unarchived => "unarchived".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryDispatchOnDemandTestAction {
    #[serde(rename = "on-demand-test")]
    OnDemandTest,
}
impl ToString for RepositoryDispatchOnDemandTestAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryDispatchOnDemandTestAction::OnDemandTest => "on-demand-test".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryImportEventStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failure")]
    Failure,
}
impl ToString for RepositoryImportEventStatus {
    fn to_string(&self) -> String {
        match *self {
            RepositoryImportEventStatus::Success => "success".to_string(),
            RepositoryImportEventStatus::Cancelled => "cancelled".to_string(),
            RepositoryImportEventStatus::Failure => "failure".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryVulnerabilityAlertCreateAction {
    #[serde(rename = "create")]
    Create,
}
impl ToString for RepositoryVulnerabilityAlertCreateAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryVulnerabilityAlertCreateAction::Create => "create".to_string(),
        }
    }
}
#[doc = "The security alert of the vulnerable dependency."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertCreateAlert {
    pub affected_package_name: String,
    pub affected_range: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismisser: Option<User>,
    pub external_identifier: String,
    pub external_reference: String,
    pub fixed_in: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghsa_id: Option<String>,
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryVulnerabilityAlertDismissAction {
    #[serde(rename = "dismiss")]
    Dismiss,
}
impl ToString for RepositoryVulnerabilityAlertDismissAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryVulnerabilityAlertDismissAction::Dismiss => "dismiss".to_string(),
        }
    }
}
#[doc = "The security alert of the vulnerable dependency."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertDismissAlert {
    pub affected_package_name: String,
    pub affected_range: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub dismiss_reason: String,
    pub dismissed_at: String,
    pub dismisser: User,
    pub external_identifier: String,
    pub external_reference: String,
    pub fixed_in: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghsa_id: Option<String>,
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RepositoryVulnerabilityAlertResolveAction {
    #[serde(rename = "resolve")]
    Resolve,
}
impl ToString for RepositoryVulnerabilityAlertResolveAction {
    fn to_string(&self) -> String {
        match *self {
            RepositoryVulnerabilityAlertResolveAction::Resolve => "resolve".to_string(),
        }
    }
}
#[doc = "The security alert of the vulnerable dependency."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct RepositoryVulnerabilityAlertResolveAlert {
    pub affected_package_name: String,
    pub affected_range: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismiss_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismissed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dismisser: Option<User>,
    pub external_identifier: String,
    pub external_reference: String,
    pub fixed_in: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ghsa_id: Option<String>,
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecretScanningAlertCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for SecretScanningAlertCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            SecretScanningAlertCreatedAction::Created => "created".to_string(),
        }
    }
}
#[doc = "The secret scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertCreatedAlert {
    pub number: i64,
    pub resolution: (),
    pub resolved_at: (),
    pub resolved_by: (),
    pub secret_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecretScanningAlertReopenedAction {
    #[serde(rename = "reopened")]
    Reopened,
}
impl ToString for SecretScanningAlertReopenedAction {
    fn to_string(&self) -> String {
        match *self {
            SecretScanningAlertReopenedAction::Reopened => "reopened".to_string(),
        }
    }
}
#[doc = "The secret scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertReopenedAlert {
    pub number: i64,
    pub resolution: (),
    pub resolved_at: (),
    pub resolved_by: (),
    pub secret_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecretScanningAlertResolvedAction {
    #[serde(rename = "resolved")]
    Resolved,
}
impl ToString for SecretScanningAlertResolvedAction {
    fn to_string(&self) -> String {
        match *self {
            SecretScanningAlertResolvedAction::Resolved => "resolved".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecretScanningAlertResolvedAlertResolution {
    #[serde(rename = "false_positive")]
    FalsePositive,
    #[serde(rename = "wontfix")]
    Wontfix,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "used_in_tests")]
    UsedInTests,
}
impl ToString for SecretScanningAlertResolvedAlertResolution {
    fn to_string(&self) -> String {
        match *self {
            SecretScanningAlertResolvedAlertResolution::FalsePositive => {
                "false_positive".to_string()
            }
            SecretScanningAlertResolvedAlertResolution::Wontfix => "wontfix".to_string(),
            SecretScanningAlertResolvedAlertResolution::Revoked => "revoked".to_string(),
            SecretScanningAlertResolvedAlertResolution::UsedInTests => "used_in_tests".to_string(),
        }
    }
}
#[doc = "The secret scanning alert involved in the event."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecretScanningAlertResolvedAlert {
    pub number: i64,
    pub resolution: SecretScanningAlertResolvedAlertResolution,
    pub resolved_at: String,
    pub resolved_by: User,
    pub secret_type: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecurityAdvisoryPerformedAction {
    #[serde(rename = "performed")]
    Performed,
}
impl ToString for SecurityAdvisoryPerformedAction {
    fn to_string(&self) -> String {
        match *self {
            SecurityAdvisoryPerformedAction::Performed => "performed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisoryIdentifiersItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisoryReferencesItem {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion {
    pub identifier: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItemPackage {
    pub ecosystem: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItem {
    pub first_patched_version:
        Option<SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion>,
    pub package: SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItemPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
}
#[doc = "The details of the security advisory, including summary, description, and severity."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPerformedSecurityAdvisory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss: Option<String>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<SecurityAdvisoryPerformedSecurityAdvisoryIdentifiersItem>,
    pub published_at: String,
    pub references: Vec<SecurityAdvisoryPerformedSecurityAdvisoryReferencesItem>,
    pub severity: String,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<SecurityAdvisoryPerformedSecurityAdvisoryVulnerabilitiesItem>,
    pub withdrawn_at: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecurityAdvisoryPublishedAction {
    #[serde(rename = "published")]
    Published,
}
impl ToString for SecurityAdvisoryPublishedAction {
    fn to_string(&self) -> String {
        match *self {
            SecurityAdvisoryPublishedAction::Published => "published".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryIdentifiersItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryReferencesItem {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion {
    pub identifier: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItemPackage {
    pub ecosystem: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItem {
    pub first_patched_version:
        Option<SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion>,
    pub package: SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItemPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
}
#[doc = "The details of the security advisory, including summary, description, and severity."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryPublishedSecurityAdvisory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss: Option<String>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<SecurityAdvisoryPublishedSecurityAdvisoryIdentifiersItem>,
    pub published_at: String,
    pub references: Vec<SecurityAdvisoryPublishedSecurityAdvisoryReferencesItem>,
    pub severity: String,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<SecurityAdvisoryPublishedSecurityAdvisoryVulnerabilitiesItem>,
    pub withdrawn_at: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SecurityAdvisoryUpdatedAction {
    #[serde(rename = "updated")]
    Updated,
}
impl ToString for SecurityAdvisoryUpdatedAction {
    fn to_string(&self) -> String {
        match *self {
            SecurityAdvisoryUpdatedAction::Updated => "updated".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisoryIdentifiersItem {
    #[serde(rename = "type")]
    pub type_: String,
    pub value: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisoryReferencesItem {
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion {
    pub identifier: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItemPackage {
    pub ecosystem: String,
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItem {
    pub first_patched_version:
        Option<SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItemFirstPatchedVersion>,
    pub package: SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItemPackage,
    pub severity: String,
    pub vulnerable_version_range: String,
}
#[doc = "The details of the security advisory, including summary, description, and severity."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SecurityAdvisoryUpdatedSecurityAdvisory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cvss: Option<String>,
    pub description: String,
    pub ghsa_id: String,
    pub identifiers: Vec<SecurityAdvisoryUpdatedSecurityAdvisoryIdentifiersItem>,
    pub published_at: String,
    pub references: Vec<SecurityAdvisoryUpdatedSecurityAdvisoryReferencesItem>,
    pub severity: String,
    pub summary: String,
    pub updated_at: String,
    pub vulnerabilities: Vec<SecurityAdvisoryUpdatedSecurityAdvisoryVulnerabilitiesItem>,
    pub withdrawn_at: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SimplePullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub self_: Link,
    pub statuses: Link,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SimplePullRequestActiveLockReason {
    #[serde(rename = "resolved")]
    Resolved,
    #[serde(rename = "off-topic")]
    OffTopic,
    #[serde(rename = "too heated")]
    TooHeated,
    #[serde(rename = "spam")]
    Spam,
}
impl ToString for SimplePullRequestActiveLockReason {
    fn to_string(&self) -> String {
        match *self {
            SimplePullRequestActiveLockReason::Resolved => "resolved".to_string(),
            SimplePullRequestActiveLockReason::OffTopic => "off-topic".to_string(),
            SimplePullRequestActiveLockReason::TooHeated => "too heated".to_string(),
            SimplePullRequestActiveLockReason::Spam => "spam".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SimplePullRequestBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SimplePullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SimplePullRequestRequestedReviewersItem {
    User(User),
    Team(Team),
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SimplePullRequestState {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
impl ToString for SimplePullRequestState {
    fn to_string(&self) -> String {
        match *self {
            SimplePullRequestState::Open => "open".to_string(),
            SimplePullRequestState::Closed => "closed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipCancelledAction {
    #[serde(rename = "cancelled")]
    Cancelled,
}
impl ToString for SponsorshipCancelledAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipCancelledAction::Cancelled => "cancelled".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipCancelledSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for SponsorshipCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipCreatedSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for SponsorshipEditedAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipEditedChangesPrivacyLevel {
    #[doc = "The `edited` event types include the details about the change when someone edits a sponsorship to change the privacy."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<SponsorshipEditedChangesPrivacyLevel>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipEditedSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipPendingCancellationAction {
    #[serde(rename = "pending_cancellation")]
    PendingCancellation,
}
impl ToString for SponsorshipPendingCancellationAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipPendingCancellationAction::PendingCancellation => {
                "pending_cancellation".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingCancellationSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipPendingTierChangeAction {
    #[serde(rename = "pending_tier_change")]
    PendingTierChange,
}
impl ToString for SponsorshipPendingTierChangeAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipPendingTierChangeAction::PendingTierChange => {
                "pending_tier_change".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingTierChangeChangesTier {
    pub from: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingTierChangeChanges {
    pub tier: SponsorshipPendingTierChangeChangesTier,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipPendingTierChangeSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SponsorshipTierChangedAction {
    #[serde(rename = "tier_changed")]
    TierChanged,
}
impl ToString for SponsorshipTierChangedAction {
    fn to_string(&self) -> String {
        match *self {
            SponsorshipTierChangedAction::TierChanged => "tier_changed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipTierChangedChangesTier {
    pub from: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipTierChangedChanges {
    pub tier: SponsorshipTierChangedChangesTier,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SponsorshipTierChangedSponsorship {
    pub created_at: String,
    pub node_id: String,
    pub privacy_level: String,
    pub sponsor: User,
    pub sponsorable: User,
    pub tier: SponsorshipTier,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StarCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for StarCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            StarCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StarDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for StarDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            StarDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventBranchesItemCommit {
    pub sha: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventBranchesItem {
    pub commit: StatusEventBranchesItemCommit,
    pub name: String,
    pub protected: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventCommitCommitTree {
    pub sha: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum StatusEventCommitCommitVerificationReason {
    #[serde(rename = "expired_key")]
    ExpiredKey,
    #[serde(rename = "not_signing_key")]
    NotSigningKey,
    #[serde(rename = "gpgverify_error")]
    GpgverifyError,
    #[serde(rename = "gpgverify_unavailable")]
    GpgverifyUnavailable,
    #[serde(rename = "unsigned")]
    Unsigned,
    #[serde(rename = "unknown_signature_type")]
    UnknownSignatureType,
    #[serde(rename = "no_user")]
    NoUser,
    #[serde(rename = "unverified_email")]
    UnverifiedEmail,
    #[serde(rename = "bad_email")]
    BadEmail,
    #[serde(rename = "unknown_key")]
    UnknownKey,
    #[serde(rename = "malformed_signature")]
    MalformedSignature,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "valid")]
    Valid,
}
impl ToString for StatusEventCommitCommitVerificationReason {
    fn to_string(&self) -> String {
        match *self {
            StatusEventCommitCommitVerificationReason::ExpiredKey => "expired_key".to_string(),
            StatusEventCommitCommitVerificationReason::NotSigningKey => {
                "not_signing_key".to_string()
            }
            StatusEventCommitCommitVerificationReason::GpgverifyError => {
                "gpgverify_error".to_string()
            }
            StatusEventCommitCommitVerificationReason::GpgverifyUnavailable => {
                "gpgverify_unavailable".to_string()
            }
            StatusEventCommitCommitVerificationReason::Unsigned => "unsigned".to_string(),
            StatusEventCommitCommitVerificationReason::UnknownSignatureType => {
                "unknown_signature_type".to_string()
            }
            StatusEventCommitCommitVerificationReason::NoUser => "no_user".to_string(),
            StatusEventCommitCommitVerificationReason::UnverifiedEmail => {
                "unverified_email".to_string()
            }
            StatusEventCommitCommitVerificationReason::BadEmail => "bad_email".to_string(),
            StatusEventCommitCommitVerificationReason::UnknownKey => "unknown_key".to_string(),
            StatusEventCommitCommitVerificationReason::MalformedSignature => {
                "malformed_signature".to_string()
            }
            StatusEventCommitCommitVerificationReason::Invalid => "invalid".to_string(),
            StatusEventCommitCommitVerificationReason::Valid => "valid".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventCommitCommitVerification {
    pub payload: Option<String>,
    pub reason: StatusEventCommitCommitVerificationReason,
    pub signature: Option<String>,
    pub verified: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventCommitCommit {
    pub author: Committer,
    pub comment_count: i64,
    pub committer: Committer,
    pub message: String,
    pub tree: StatusEventCommitCommitTree,
    pub url: String,
    pub verification: StatusEventCommitCommitVerification,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventCommitParentsItem {
    pub html_url: String,
    pub sha: String,
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct StatusEventCommit {
    pub author: Option<User>,
    pub comments_url: String,
    pub commit: StatusEventCommitCommit,
    pub committer: Option<User>,
    pub html_url: String,
    pub node_id: String,
    pub parents: Vec<StatusEventCommitParentsItem>,
    pub sha: String,
    pub url: String,
}
#[doc = "The new state. Can be `pending`, `success`, `failure`, or `error`."]
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamParentPrivacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}
impl ToString for TeamParentPrivacy {
    fn to_string(&self) -> String {
        match *self {
            TeamParentPrivacy::Open => "open".to_string(),
            TeamParentPrivacy::Closed => "closed".to_string(),
            TeamParentPrivacy::Secret => "secret".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamParent {
    #[doc = "Description of the team"]
    pub description: Option<String>,
    pub html_url: String,
    #[doc = "Unique identifier of the team"]
    pub id: i64,
    pub members_url: String,
    #[doc = "Name of the team"]
    pub name: String,
    pub node_id: String,
    #[doc = "Permission that the team will have for its repositories"]
    pub permission: String,
    pub privacy: TeamParentPrivacy,
    pub repositories_url: String,
    pub slug: String,
    #[doc = "URL for the team"]
    pub url: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamPrivacy {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "secret")]
    Secret,
}
impl ToString for TeamPrivacy {
    fn to_string(&self) -> String {
        match *self {
            TeamPrivacy::Open => "open".to_string(),
            TeamPrivacy::Closed => "closed".to_string(),
            TeamPrivacy::Secret => "secret".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamAddedToRepositoryAction {
    #[serde(rename = "added_to_repository")]
    AddedToRepository,
}
impl ToString for TeamAddedToRepositoryAction {
    fn to_string(&self) -> String {
        match *self {
            TeamAddedToRepositoryAction::AddedToRepository => "added_to_repository".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamCreatedAction {
    #[serde(rename = "created")]
    Created,
}
impl ToString for TeamCreatedAction {
    fn to_string(&self) -> String {
        match *self {
            TeamCreatedAction::Created => "created".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamDeletedAction {
    #[serde(rename = "deleted")]
    Deleted,
}
impl ToString for TeamDeletedAction {
    fn to_string(&self) -> String {
        match *self {
            TeamDeletedAction::Deleted => "deleted".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamEditedAction {
    #[serde(rename = "edited")]
    Edited,
}
impl ToString for TeamEditedAction {
    fn to_string(&self) -> String {
        match *self {
            TeamEditedAction::Edited => "edited".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesDescription {
    #[doc = "The previous version of the description if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesName {
    #[doc = "The previous version of the name if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesPrivacy {
    #[doc = "The previous version of the team's privacy if the action was `edited`."]
    pub from: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesRepositoryPermissionsFrom {
    #[doc = "The previous version of the team member's `admin` permission on a repository, if the action was `edited`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[doc = "The previous version of the team member's `pull` permission on a repository, if the action was `edited`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull: Option<bool>,
    #[doc = "The previous version of the team member's `push` permission on a repository, if the action was `edited`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub push: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesRepositoryPermissions {
    pub from: TeamEditedChangesRepositoryPermissionsFrom,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChangesRepository {
    pub permissions: TeamEditedChangesRepositoryPermissions,
}
#[doc = "The changes to the team if the action was `edited`."]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct TeamEditedChanges {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<TeamEditedChangesDescription>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TeamEditedChangesName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy: Option<TeamEditedChangesPrivacy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<TeamEditedChangesRepository>,
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TeamRemovedFromRepositoryAction {
    #[serde(rename = "removed_from_repository")]
    RemovedFromRepository,
}
impl ToString for TeamRemovedFromRepositoryAction {
    fn to_string(&self) -> String {
        match *self {
            TeamRemovedFromRepositoryAction::RemovedFromRepository => {
                "removed_from_repository".to_string()
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum UserType {
    Bot,
    User,
    Organization,
}
impl ToString for UserType {
    fn to_string(&self) -> String {
        match *self {
            UserType::Bot => "Bot".to_string(),
            UserType::User => "User".to_string(),
            UserType::Organization => "Organization".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum WatchStartedAction {
    #[serde(rename = "started")]
    Started,
}
impl ToString for WatchStartedAction {
    fn to_string(&self) -> String {
        match *self {
            WatchStartedAction::Started => "started".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum WorkflowRunCompletedAction {
    #[serde(rename = "completed")]
    Completed,
}
impl ToString for WorkflowRunCompletedAction {
    fn to_string(&self) -> String {
        match *self {
            WorkflowRunCompletedAction::Completed => "completed".to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum WorkflowRunRequestedAction {
    #[serde(rename = "requested")]
    Requested,
}
impl ToString for WorkflowRunRequestedAction {
    fn to_string(&self) -> String {
        match *self {
            WorkflowRunRequestedAction::Requested => "requested".to_string(),
        }
    }
}
