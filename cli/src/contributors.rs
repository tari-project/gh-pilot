use github_pilot_api::{provider_traits::Contributors, GithubProvider};
use log::warn;

use crate::pretty_print::pretty_table;

pub async fn run_contributor_cmd(provider: &GithubProvider, owner: &str, repo: &str) -> Result<(), ()> {
    let contributors = provider.get_contributors(owner, repo).await.map_err(|e| {
        warn!("⏩ Error fetching contributors: {}", e.to_string());
    })?;
    println!("👀 {} Contributors for {owner}/{repo}:", contributors.len());
    let mut table = pretty_table("Name", "Contributions");
    for contributor in contributors {
        table.add_row([
            contributor.login.as_str(),
            contributor.contributions.to_string().as_str(),
        ]);
    }
    println!("{table}");
    Ok(())
}
