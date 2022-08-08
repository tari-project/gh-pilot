extern crate core;

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
use gh_pilot::GithubProvider;
use gh_pilot::mock_provider::MockUserProvider;
use crate::issue::run_issue_cmd;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    let user_provider = MockUserProvider::default();
    let github_provider = GithubProvider::default();
    let mut context = Context::default();
    context.use_user_provider(&user_provider);
    context.use_pr_provider(&github_provider);
    context.use_isue_provider(&github_provider);
    let owner = cli.owner.as_str();
    let repo = cli.repo.as_str();
    match &cli.command {
        Some(Commands::User { profile }) => run_user_cmd(&context, profile).await,
        Some(Commands::PullRequest { number, }) => run_pr_cmd(&context, owner,repo, *number).await,
        Some(Commands::Issue { number, }) => run_issue_cmd(&context, owner, repo, *number).await,
        _ => {
            println!("Not yet supported");
            Ok(())
        }
    }
}
