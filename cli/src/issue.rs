use crate::pretty_print::{cc, pretty_table};
use crate::Context;
use comfy_table::{Cell, Color, Row, Table};
use gh_pilot::github::models::{Issue, Label};
use std::str::FromStr;

pub async fn run_issue_cmd(ctx: &Context, number: u64) -> Result<(), ()> {
    let provider = ctx.issue_provider();
    match provider.fetch_issue(number).await {
        Ok(issue) => pretty_print(issue),
        Err(e) => println!("Error fetching issue: {}", e.to_string()),
    }
    Ok(())
}

fn pretty_print(issue: Issue) {
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
        let desc = label
            .description
            .as_ref()
            .map(|d| d.as_str())
            .unwrap_or_default();
        row.add_cell(cc(color, label.name.as_str()))
            .add_cell(Cell::new(desc));
        table.add_row(row);
    })
}
