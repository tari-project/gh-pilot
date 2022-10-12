use serde::{Deserialize, Serialize};

use crate::models::{Commit, Committer, CommonEventFields};

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
