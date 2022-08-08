use crate::pretty_print::{add_labels, pretty_table};
use crate::Context;
use gh_pilot::github::models::{Issue};

pub async fn run_issue_cmd(ctx: &Context<'_>, owner: &str, repo: &str, number: u64) -> Result<(), ()> {
    if let Some(provider) = ctx.issue_provider() {
        match provider.fetch_issue(owner, repo, number).await {
            Ok(issue) => pretty_print(issue),
            Err(e) => println!("Error fetching issue: {}", e.to_string()),
        }
    } else {
        println!("No Issue provider was installed");
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


