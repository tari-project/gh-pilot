use github_pilot_api::{models::PullRequest, provider_traits::PullRequestProvider};
use log::*;

use crate::pretty_print::{add_labels, pretty_table};

pub async fn run_pr_cmd(provider: &dyn PullRequestProvider, owner: &str, repo: &str, number: u64) -> Result<(), ()> {
    match provider.fetch_pull_request(owner, repo, number).await {
        Ok(pr) => pretty_print(pr),
        Err(e) => warn!("⏩ Error fetching PR: {}", e.to_string()),
    }
    Ok(())
}

fn pretty_print(pr: PullRequest) {
    let mut table = pretty_table("Title", pr.title.as_str());
    table
        .add_row(["URL", pr.url.as_ref()])
        .add_row(["State", pr.state.to_string().as_str()])
        .add_row(["Merged", if matches!(pr.merged, Some(true)) { "Yes" } else { "No" }])
        .add_row(["Labels"]);
    add_labels(&mut table, &pr.labels);
    println!("{table}");
    println!("{}", pr.body.unwrap_or_else(|| "No body provided".into()));
}
