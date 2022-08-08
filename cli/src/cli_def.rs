use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
    #[clap(short, long, default_value = "tari-project")]
    pub owner: String,
    #[clap(short, long, default_value = "tari")]
    pub repo: String,
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
    Issue {
        #[clap(short, long)]
        number: u64,
    },
}
