use github_pilot_api::{
    models::DateTime,
    wrappers::{GithubHandle, RepoId},
    GithubProvider,
};

use crate::pilot_command::{
    generate_activity_reports,
    issue::IssueCmd,
    labels::LabelCmd,
    pull_request::PrCmd,
    user::{run_contributor_cmd, run_user_cmd},
};

/// This is the final specification of the command to pass through to the API. All modifications to the request have
/// been processed, including, command-line arguments, environment variables, and UI prompts.
///
/// The sub-commands get run in delegated functions in this module
#[derive(Debug)]
pub enum PilotCommand {
    /// ActivityReport
    ActivityReport(Vec<GithubHandle>, DateTime),
    /// User command
    User(GithubHandle),
    /// Fetches a pull request
    PullRequest(PrCmd),
    /// Query or manipulate an issue
    Issue(IssueCmd),
    /// Manipulate labels on a repo
    Labels(LabelCmd),
    /// List contributors to the repo
    Contributors(RepoId),
    /// Do Nothing
    NoOp,
}

impl PilotCommand {
    pub async fn execute(self, provider: &GithubProvider) -> Result<(), String> {
        match self {
            PilotCommand::User(h) => run_user_cmd(provider, h).await,
            PilotCommand::PullRequest(cmd) => cmd.execute(provider).await,
            PilotCommand::Issue(cmd) => cmd.execute(provider).await,
            PilotCommand::Labels(cmd) => cmd.execute(provider).await,
            PilotCommand::Contributors(ref id) => run_contributor_cmd(provider, id).await,
            PilotCommand::NoOp => Ok(()),
            PilotCommand::ActivityReport(ids, since) => generate_activity_reports(provider, ids, since).await,
        }
    }
}
