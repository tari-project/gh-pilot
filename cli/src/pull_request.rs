use std::str::FromStr;
use comfy_table::{Cell, Color, Row, Table};
use crate::Context;
use gh_pilot::github::models::{Label, PullRequest};
use crate::pretty_print::{cc, pretty_table};

pub async fn run_pr_cmd(ctx: &Context, owner: &str, repo: &str, number: u64) -> Result<(), ()> {
    let provider = ctx.pull_request_provider();
    match provider.fetch_pull_request(owner, repo, number).await {
        Ok(pr) => pretty_print(pr),
        Err(e) => println!("Error fetching pr: {}", e.to_string())
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

fn add_labels(table: &mut Table, labels: &[Label]) {
    table.add_row(&["Labels"]);
    labels.iter().for_each(|label| {
        let mut row = Row::new();
        let color = Color::from_str(label.color.as_str()).unwrap_or_else(|_| Color::White);
        let desc = label.description.as_ref().map(|d| d.as_str()).unwrap_or_default();
        row
            .add_cell(cc(color, label.name.as_str()))
            .add_cell(Cell::new(desc));
        table.add_row(row);
    })
}