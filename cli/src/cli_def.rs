use std::fmt::Display;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    /// The organisation or repository owner (default: tari-project)
    #[clap(short, long, default_value = "tari-project")]
    pub owner: String,
    /// The repository to query (default: tari)
    #[clap(short, long, default_value = "tari")]
    pub repo: String,
    #[clap(short = 'u', long = "user", env = "GH_PILOT_USERNAME")]
    pub user_name: Option<String>,
    #[clap(short = 'a', long = "auth", env = "GH_PILOT_AUTH_TOKEN")]
    pub auth_token: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// User command
    User {
        /// displays a user's profile
        #[clap(short, long, action)]
        profile: String,
    },
    /// Fetches a pull request
    PullRequest {
        #[clap(short, long)]
        number: u64,
    },
    /// Query or manipulate an issue
    Issue {
        #[clap(short, long)]
        number: u64,
        #[clap(subcommand)]
        sub_command: IssueCommand,
    },
    /// Manipulate labels on a repo
    Labels {
        #[clap(subcommand)]
        sub_command: LabelCommand,
    },
}

#[derive(Debug, Subcommand)]
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
        #[clap(short, long, arg_enum, value_parser)]
        format: OutputFormat,
    },
    /// Create a new label
    #[clap(name = "create")]
    Create {
        /// The name of the label
        #[clap(short, long)]
        name: String,
        /// The color of the label
        #[clap(short, long)]
        color: Option<String>,
        /// The description of the label
        #[clap(short, long)]
        description: Option<String>,
    },
    /// Delete a label
    #[clap(name = "delete")]
    Delete {
        /// The name of the label
        #[clap(short, long)]
        label: String,
    },
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
        label: String,
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

#[derive(Debug, Subcommand)]
pub enum IssueCommand {
    /// Fetches an issue
    Fetch,
    /// Adds a label to an issue
    AddLabel(LabelArg),
    /// Removes a label from an issue
    RemoveLabel(LabelArg),
}

#[derive(Debug, Args)]
pub struct LabelArg {
    pub label: String,
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

#[cfg(test)]
mod test {
    use crate::Cli;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
