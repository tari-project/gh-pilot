use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
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
        #[clap(short, long, default_value="tari-project")]
        owner: String,
        #[clap(short, long, default_value="tari")]
        repo: String,
        #[clap(short, long)]
        number: u64,
    }
}
