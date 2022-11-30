mod cli_def;
mod cli_prompts;
mod pilot_command;
mod pretty_print;

use std::str::FromStr;

use clap::Parser;
use cli_def::Cli;
use dotenv::dotenv;
use github_pilot_api::{wrappers::RepoId, GithubProvider};
use log::*;

#[tokio::main]
async fn main() -> Result<(), String> {
    dotenv().ok();
    env_logger::init();
    let mut cli: Cli = Cli::parse();
    let _ = try_merge_current_repo_settings(&mut cli)
        .map_err(|e| debug!("Not using current directory's git settings: {}", e));
    // Use the UI to fill in missing arguments if needed
    let provider = setup_github_api(&cli);
    let cmd = cli.as_pilot_command(&provider).await?;
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

fn try_merge_current_repo_settings(cli: &mut Cli) -> Result<(), String> {
    let current_dir_repo =
        git2::Repository::discover(".").map_err(|e| format!("Current directory is not a git repository:{}", e))?;
    let remote = current_dir_repo
        .find_remote("origin")
        .map_err(|e| format!("Current directory does not have a remote:{}", e))?;
    let remote_url = remote.url().ok_or_else(|| "Remote does not have a URL".to_string())?;
    let url = if remote_url.starts_with("http") {
        url::Url::parse(remote_url)
            .map_err(|e| format!("Not a valid remote url:{}", e))?
            .path()
            .to_string()
    } else if remote_url.starts_with("git") {
        let path: Vec<&str> = remote_url.split(':').collect();
        path.get(1)
            .ok_or_else(|| "Invalid remote git url".to_string())?
            .to_string()
    } else {
        return Err("Unknown remote url format".to_string());
    };

    if let Ok(repo_id) = RepoId::from_str(&url) {
        if cli.repo.is_none() {
            cli.repo = Some(repo_id.repo);
        }
        if cli.owner.is_none() {
            cli.owner = Some(repo_id.owner);
        }
    }
    Ok(())
}
