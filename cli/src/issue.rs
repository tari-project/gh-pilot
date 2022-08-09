use crate::pretty_print::{add_labels, pretty_table};
use crate::Context;
use gh_pilot::github::models::{Issue};
use gh_pilot::models::IssueId;
use crate::cli_def::IssueCommand;
use log::*;

pub async fn run_issue_cmd(ctx: &Context<'_>, owner: &str, repo: &str, number: u64, cmd: &IssueCommand) -> Result<(), ()> {
    if ctx.issue_provider().is_none() {
        warn!("No Issue provider was installed");
        return Err(());
    }
    let id = IssueId::new(owner, repo, number);
    let provider = ctx.issue_provider().unwrap();
    match cmd {
        IssueCommand::Fetch => match provider.fetch_issue(&id).await {
            Ok(issue) => pretty_print(issue),
            Err(e) => warn!("Error fetching issue: {}", e.to_string()),
        },
        IssueCommand::AddLabel(l) => match provider.add_label(&id, l.label.as_str()).await {
            Ok(_) => info!("{} added to issue {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("Error adding label to issue: {}", e.to_string()),
        },
        IssueCommand::RemoveLabel(l) => match provider.remove_label(&id, l.label.as_str()).await {
            Ok(_) => info!("{} removed from issue {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("Error removing label from issue: {}", e.to_string()),
        },
    }
    Ok(())
}

fn pretty_print(issue: Issue) {
    let mut table = pretty_table("Title", issue.title.as_str());
    table
        .add_row(&["URL", issue.url.as_ref()])
        .add_row(&["State", issue.state.to_string().as_str()]);
    add_labels(&mut table, &issue.labels);
    println!("{table}");
    println!("{}", issue.body.unwrap_or_default());
}


