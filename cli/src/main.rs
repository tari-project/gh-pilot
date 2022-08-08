mod cli_def;
mod context;
mod issue;
mod pretty_print;
mod pull_request;
mod user;

use crate::context::Context;
use crate::pull_request::run_pr_cmd;
use crate::user::run_user_cmd;
use clap::Parser;
use cli_def::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    // let context = Context::mock();
    let context = Context::custom_context();
    match &cli.command {
        Some(Commands::User { profile }) => run_user_cmd(&context, profile).await,
        Some(Commands::PullRequest {
            owner,
            repo,
            number,
        }) => run_pr_cmd(&context, owner, repo, *number).await,
        _ => {
            println!("Not yet supported");
            Ok(())
        }
    }
}
