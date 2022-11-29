use github_pilot_api::{models::Issue, provider_traits::IssueProvider, wrappers::IssueId};
use log::*;

use crate::pretty_print::{add_labels, pretty_table};

#[derive(Debug)]
pub enum IssueCmd {
    /// Fetches an issue
    Fetch(IssueId),
    /// Adds a label to an issue
    AddLabel(IssueId, String),
    /// Removes a label from an issue
    RemoveLabel(IssueId, String),
}

impl IssueCmd {
    pub async fn execute(self, provider: &dyn IssueProvider) -> Result<(), String> {
        match self {
            IssueCmd::Fetch(id) => match provider.fetch_issue(&id).await {
                Ok(issue) => {
                    pretty_print(issue);
                    Ok(())
                },
                Err(e) => Err(format!("📒 Error fetching issue {id}: {e}")),
            },
            IssueCmd::AddLabel(id, l) => match provider.add_label(&id, l.as_str()).await {
                Ok(_) => Ok(info!("🏷 '{l}' added to issue {id}")),
                Err(e) => Err(format!("🏷 Error adding label to issue {id}: {e}")),
            },
            IssueCmd::RemoveLabel(id, l) => match provider.remove_label(&id, l.as_str(), false).await {
                Ok(true) => Ok(info!("🏷 '{l}' removed from issue {id}")),
                Ok(false) => Ok(info!("🏷 '{l}' was not present on issue {id}")),
                Err(e) => Err(format!("🏷 Error removing label from issue {id}: {e}")),
            },
        }
    }
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
