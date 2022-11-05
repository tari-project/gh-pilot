use comfy_table::{presets::UTF8_BORDERS_ONLY, Cell, Color, ContentArrangement, Row, Table};
use github_pilot_api::{
    models::SimpleUser,
    provider_traits::{Contributors, UserProvider},
    wrappers::{GithubHandle, RepoId},
    GithubProvider,
};
use log::*;

use crate::pretty_print::pretty_table;

pub async fn run_user_cmd(provider: &dyn UserProvider, handle: GithubHandle) -> Result<(), String> {
    debug!("👨 Fetching user..{}", handle.as_ref());
    let details = provider
        .fetch_details(&handle)
        .await
        .map_err(|_e| format!("Error fetching {}", handle.as_ref()))?;
    match details {
        Some(p) => pretty_print(&p),
        None => info!("👨 User {} was not found", handle.as_ref()),
    }
    Ok(())
}

pub async fn run_contributor_cmd(provider: &GithubProvider, repo: &RepoId) -> Result<(), String> {
    let contributors = provider
        .fetch_contributors(repo.owner(), repo.repo())
        .await
        .map_err(|e| format!("⏩ Error fetching contributors: {e}"))?;
    println!("👀 {} Contributors for {repo}:", contributors.len());
    let mut table = pretty_table(&["Name", "Contributions"]);
    for contributor in contributors {
        table.add_row([
            contributor.login.as_str(),
            contributor.contributions.to_string().as_str(),
        ]);
    }
    println!("{table}");
    Ok(())
}

fn pretty_print(user: &SimpleUser) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_BORDERS_ONLY)
        .set_content_arrangement(ContentArrangement::Dynamic);
    let mut name_row = Row::new();
    name_row
        .add_cell(cc(Color::Green, "User name"))
        .add_cell(cc(Color::Green, user.login.as_str()));
    table
        .add_row(name_row)
        .add_row(["Type", user.name.as_deref().unwrap_or("Unavailable")])
        .add_row(["URL", user.url.as_ref()]);
    println!("{table}");
}

fn cc(color: Color, val: &str) -> Cell {
    Cell::new(val).fg(color)
}
