use github_pilot_api::{models::Issue, provider_traits::IssueProvider, wrappers::IssueId};
use log::*;

use crate::{
    cli_def::IssueCommand,
    pretty_print::{add_labels, pretty_table},
};

pub async fn run_issue_cmd(
    provider: &dyn IssueProvider,
    owner: &str,
    repo: &str,
    number: u64,
    cmd: IssueCommand,
) -> Result<(), ()> {
    let id = IssueId::new(owner, repo, number);
    match cmd {
        IssueCommand::Fetch => match provider.fetch_issue(&id).await {
            Ok(issue) => pretty_print(issue),
            Err(e) => warn!("📒 Error fetching issue: {}", e.to_string()),
        },
        IssueCommand::AddLabel(l) => match provider.add_label(&id, l.label.as_str()).await {
            Ok(_) => info!("🏷 '{}' added to issue {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("🏷 Error adding label to issue: {}", e.to_string()),
        },
        IssueCommand::RemoveLabel(l) => match provider.remove_label(&id, l.label.as_str(), false).await {
            Ok(true) => info!("🏷 '{}' removed from issue {}/{}", l.label, id.repo, id.number),
            Ok(false) => info!("🏷 '{}' was not present on issue {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("🏷 Error removing label from issue: {}", e.to_string()),
        },
    }
    Ok(())
}

fn pretty_print(issue: Issue) {
    let mut table = pretty_table(&["Title", issue.title.as_str()]);
    table
        .add_row(["URL", issue.url.as_ref()])
        .add_row(["State", issue.state.to_string().as_str()])
        .add_row(["Labels"]);
    add_labels(&mut table, &issue.labels);
    println!("{table}");
    println!("{}", issue.body.unwrap_or_default());
}
