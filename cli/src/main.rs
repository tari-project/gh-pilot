mod cli_def;
mod cli_prompts;
mod pilot_command;
mod pretty_print;

use clap::Parser;
use cli_def::{Cli};
use dotenv::dotenv;
use github_pilot_api::{GithubProvider};
use log::*;



#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    env_logger::init();
    let mut cli: Cli = Cli::parse();
    // Use the UI to fill in missing arguments if needed
    let provider = setup_github_api(&cli);
    let cmd = cli.into_pilot_command(&provider).await?;
    cmd.execute(&provider).await
}

fn setup_github_api(cli: &Cli) -> GithubProvider {
    match (cli.user_name.as_ref(), cli.auth_token.as_ref()) {
        (Some(u), Some(a)) => {
            info!("🚀 Ignition! Launching Github Pilot in Authenticated Mode.");
            GithubProvider::new(u.as_str(), a.as_str())
        },
        (Some(_), None) => {
            warn!("🚀 Username was set, but auth token was not provided. Defaulting to unauthenticated mode");
            GithubProvider::default()
        },
        (None, Some(_)) => {
            warn!("🚀 Auth token was set, but username was not provided. Defaulting to unauthenticated mode");
            GithubProvider::default()
        },
        (None, None) => {
            info!("🚀 No credentials provided. Launching Github Pilot in unauthenticated mode.");
            GithubProvider::default()
        },
    }
}
