//! The CLI arguments definition.
//!
//! Note: Almost EVERYTHING here is WRAPPED IN AN OPTION.
//!
//! Even if an argument is always required in a query, put it in an Option. Why?
//!
//! This is so that the user prompts module can ask the user for required info they didn't feel like putting in on
//! the command line.
//!
//! The precedence order is
//!
//! `ENV? -> cli arg? -> User prompt? -> None or Panic`
use std::fmt::Display;

use clap::{Args, Parser, Subcommand, ValueEnum};
use github_pilot_api::models_plus::MergeMethod;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    /// The organisation or repository owner.
    #[clap(short, long, env = "GH_PILOT_OWNER")]
    pub owner: Option<String>,
    /// The repository to query.
    #[clap(short, long, env = "GH_PILOT_REPO")]
    pub repo: Option<String>,
    /// The user associated with the authentication token.
    #[clap(short = 'u', long = "user", env = "GH_PILOT_USERNAME")]
    #[arg(hide_env_values = true)]
    pub user_name: Option<String>,
    /// Your Github API authentication token.
    #[clap(short = 'a', long = "auth", env = "GH_PILOT_AUTH_TOKEN")]
    #[arg(hide_env_values = true)]
    pub auth_token: Option<String>,
    /// If set, you will not be prompted for any missing info. Useful in scripts
    #[clap(short = 'x', long = "non-interactive", env = "GH_PILOT_NON_INTERACTIVE")]
    pub non_interactive: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// User command
    User {
        /// displays a user's profile
        #[clap(short, long, action)]
        profile: Option<String>,
    },
    /// Fetches a pull request
    #[clap(alias = "pr")]
    PullRequest {
        #[clap(short, long)]
        number: Option<u64>,
        #[clap(subcommand)]
        sub_command: PullRequestCommand,
    },
    /// Query or manipulate an issue
    #[clap(alias = "iss")]
    Issue {
        #[clap(short, long)]
        number: Option<u64>,
        #[clap(subcommand)]
        sub_command: IssueCommand,
    },
    /// Manipulate labels on a repo
    #[clap(alias = "lab")]
    Labels {
        #[clap(subcommand)]
        sub_command: LabelCommand,
    },
    /// List contributors to the repo
    #[clap(alias = "users")]
    Contributors,
    /// Generate an activity report for the given user(s)
    ActivityReport(ActivityReportOptions),
}

#[derive(Debug, Clone, Subcommand)]
pub enum LabelCommand {
    /// List all labels on a repo
    #[clap(name = "list")]
    List {
        /// The page number to fetch
        #[clap(short, long)]
        page: Option<usize>,
        /// The number of labels to fetch per page
        #[clap(short = 'n', long)]
        per_page: Option<usize>,
        /// The format we should display the results in.
        #[clap(short, long, value_parser, default_value = "txt")]
        format: OutputFormat,
    },
    /// Create a new label
    #[clap(name = "create")]
    Create {
        /// The name of the label
        #[clap(short, long)]
        name: Option<String>,
        /// The color of the label
        #[clap(short, long)]
        color: Option<String>,
        /// The description of the label
        #[clap(short, long)]
        description: Option<String>,
    },
    /// Delete a label
    #[clap(name = "delete")]
    Delete(LabelArg),
    /// Assign labels to a repo
    #[clap(name = "assign")]
    Assign {
        /// A path to a file containing label definitions
        #[clap(short = 'f', long = "file")]
        labels_file: String,
    },
    /// Edit an existing label
    #[clap(name = "edit")]
    Edit {
        /// The name of the label
        #[clap(short, long)]
        label: Option<String>,
        /// The new name of the label
        #[clap(short, long)]
        name: Option<String>,
        /// The new color of the label
        #[clap(short, long)]
        color: Option<String>,
        /// The new description of the label
        #[clap(short, long)]
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum IssueCommand {
    /// Fetches an issue
    Fetch,
    /// Adds a label to an issue
    AddLabel(LabelArg),
    /// Removes a label from an issue
    RemoveLabel(LabelArg),
    /// Fetch comments for this issue
    Comments,
    /// Add a comment to this issue
    #[clap(alias = "ac")]
    AddComment(CommentArg),
}

#[derive(Debug, Clone, Args)]
pub struct CommentArg {
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct ActivityReportOptions {
    /// The date from which to calculate activity
    #[clap(long, short = 's')]
    pub since: Option<String>,
    /// The Github User id to generate a report for. Can be specified multiple times.
    #[clap(conflicts_with = "user_file_path", long)]
    pub id: Option<Vec<String>>,
    /// A path to a file containing Github user ids, one id per line.
    #[clap(long = "userfile", short = 'f')]
    pub user_file_path: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct LabelArg {
    #[clap(short, long)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    #[clap(name = "txt")]
    Text,
    #[clap(name = "json")]
    Json,
    #[clap(name = "yml")]
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Text
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Text => write!(f, "txt"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Yaml => write!(f, "yml"),
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum PullRequestCommand {
    /// Fetches a PR
    Fetch,
    /// Adds a label to a PR
    AddLabel(LabelArg),
    /// Removes a label from a PR
    RemoveLabel(LabelArg),
    /// Retrieve the comments and review comment threads for a PR
    Comments,
    /// Try to merge the PR
    Merge(MergeArgs),
    /// Fetch review statistics
    Reviews,
    /// Fetch last check run status (The result of the checks that are configured to run after each PR change).
    Check,
    /// Add a comment to this PR
    #[clap(alias = "ac")]
    AddComment(CommentArg),
}

#[derive(Debug, Clone, Args)]
pub struct MergeArgs {
    /// Override the title for the commit message
    #[clap(short = 't', long = "title")]
    pub commit_title: Option<String>,
    /// Override the commit message for the merge
    #[clap(long = "message")]
    pub commit_message: Option<String>,
    /// Require the HEAD to have this SHA value before allowing a merge
    pub sha: Option<String>,
    /// Specify the merge method. Can be one of: merge, rebase, or squash. Default is merge.
    #[clap(short = 'm', long = "method")]
    pub merge_method: Option<MergeMethod>,
}

#[cfg(test)]
mod test {
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
