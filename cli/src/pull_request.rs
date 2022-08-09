use crate::pretty_print::{add_labels, pretty_table};
use crate::Context;
use gh_pilot::ghp_api::models::{PullRequest};
use log::*;

pub async fn run_pr_cmd(ctx: &Context<'_>, owner: &str, repo: &str, number: u64) -> Result<(), ()> {
    if let Some(provider) = ctx.pull_request_provider() {
        match provider.fetch_pull_request(owner, repo, number).await {
            Ok(pr) => pretty_print(pr),
            Err(e) => warn!("Error fetching pr: {}", e.to_string()),
        }
    } else {
        warn!("No PR provider installed.");
    }
    Ok(())
}

fn pretty_print(pr: PullRequest) {
    let mut table = pretty_table("Title", pr.title.as_str());
    table
        .add_row(&["URL", pr.url.as_ref()])
        .add_row(&["State", pr.state.to_string().as_str()])
        .add_row(&["Merged", if pr.merged { "Yes" } else { "No" }]);
    add_labels(&mut table, &pr.labels);
    println!("{table}");
    println!("{}", pr.body);
}

