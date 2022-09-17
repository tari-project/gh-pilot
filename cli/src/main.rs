mod cli_def;
mod issue;
mod pretty_print;
mod pull_request;
mod user;

use clap::Parser;
use cli_def::{Cli, Commands};
use github_pilot_api::GithubProvider;
use log::*;

use crate::{issue::run_issue_cmd, pull_request::run_pr_cmd, user::run_user_cmd};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env_logger::init();
    let cli = Cli::parse();
    let provider = setup_github_api(&cli);
    let owner = cli.owner.as_str();
    let repo = cli.repo.as_str();
    match &cli.command {
        Commands::User { profile } => run_user_cmd(&provider, profile).await,
        Commands::PullRequest { number } => run_pr_cmd(&provider, owner, repo, *number).await,
        Commands::Issue { number, sub_command } => run_issue_cmd(&provider, owner, repo, *number, sub_command).await,
    }
}

fn setup_github_api(cli: &Cli) -> GithubProvider {
    match (cli.user_name.as_ref(), cli.auth_token.as_ref()) {
        (Some(u), Some(a)) => {
            info!("Ignition! Launching Github Pilot in Authenticated Mode.");
            GithubProvider::new(u.as_str(), a.as_str())
        },
        (Some(_), None) => {
            warn!("Username was set, but auth token was not provided. Defaulting to unauthenticated mode");
            GithubProvider::default()
        },
        (None, Some(_)) => {
            warn!("Auth token was set, but username was not provided. Defaulting to unauthenticated mode");
            GithubProvider::default()
        },
        (None, None) => {
            info!("No credentials provided. Launching Github Pilot in unauthenticated mode.");
            GithubProvider::default()
        },
    }
}
