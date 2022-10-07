use github_pilot_api::{
    graphql::{review_counts::ReviewCounts, CheckRunStatus, PullRequestComments},
    models::PullRequest,
    provider_traits::{
        CheckRunStatusProvider,
        IssueProvider,
        PullRequestCommentsProvider,
        PullRequestProvider,
        PullRequestReviewSummary,
    },
    wrappers::IssueId,
    GithubProvider,
};
use log::*;

use crate::{
    cli_def::{MergeArgs, PullRequestCommand},
    pretty_print::{add_labels, pretty_table},
};

pub async fn run_pr_cmd(
    provider: &GithubProvider,
    owner: &str,
    repo: &str,
    number: u64,
    cmd: PullRequestCommand,
) -> Result<(), ()> {
    let id = IssueId::new(owner, repo, number);
    match cmd {
        PullRequestCommand::Fetch => fetch_pr(provider, id).await,
        PullRequestCommand::AddLabel(l) => match provider.add_label(&id, l.label.as_str()).await {
            Ok(_) => info!("⏩ '{}' added to PR {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("⏩ Error adding label to PR: {}", e.to_string()),
        },
        PullRequestCommand::RemoveLabel(l) => match provider.remove_label(&id, l.label.as_str(), false).await {
            Ok(true) => info!("⏩ '{}' removed from PR {}/{}", l.label, id.repo, id.number),
            Ok(false) => info!("⏩ '{}' was not present on PR {}/{}", l.label, id.repo, id.number),
            Err(e) => warn!("⏩ Error removing label from PR: {e}"),
        },
        PullRequestCommand::Comments => fetch_comments(provider, id).await,
        PullRequestCommand::Merge(params) => merge_pull_request(provider, &id, params).await,
        PullRequestCommand::Reviews => fetch_review_summary(provider, id).await,
        PullRequestCommand::Check => fetch_last_check_run(provider, id).await,
    }
    Ok(())
}

async fn fetch_pr(provider: &dyn PullRequestProvider, id: IssueId) {
    match provider
        .fetch_pull_request(id.owner.as_str(), id.repo.as_str(), id.number)
        .await
    {
        Ok(pr) => pretty_print(pr),
        Err(e) => warn!("⏩ Error fetching PR: {e}"),
    }
}

async fn fetch_comments(provider: &dyn PullRequestCommentsProvider, id: IssueId) {
    match provider.fetch_pull_request_comments(&id).await {
        Ok(comments) => print_comments(comments),
        Err(e) => warn!("⏩ Error fetching PR comments: {e}"),
    }
}

async fn fetch_review_summary(provider: &dyn PullRequestReviewSummary, id: IssueId) {
    match provider.fetch_review_summary(&id).await {
        Ok(summary) => print_review_summary(&summary),
        Err(e) => warn!("⏩👀 Error fetching PR review summary: {e}"),
    }
}

async fn fetch_last_check_run(provider: &dyn CheckRunStatusProvider, id: IssueId) {
    match provider.fetch_check_run(&id).await {
        Ok(run_status) => print_run_status(&run_status),
        Err(e) => warn!("⏩✅ Error fetching PR review summary: {e}"),
    }
}

async fn merge_pull_request(provider: &dyn PullRequestProvider, id: &IssueId, params: MergeArgs) {
    match provider.merge_pull_request(id, params.into()).await {
        Ok(r) => {
            if r.merged {
                info!("⏩ PR {id} was merged successfully. {}", r.message);
            } else {
                warn!("⏩ The PR {id} was NOT merged. {}", r.message);
            }
        },
        Err(e) => warn!("⏩ Error merging PR: {e}"),
    }
}

fn pretty_print(pr: PullRequest) {
    let mut table = pretty_table(&["Title", pr.title.as_str()]);
    table
        .add_row(["URL", pr.url.as_ref()])
        .add_row(["State", pr.state.to_string().as_str()])
        .add_row(["Merged", if matches!(pr.merged, Some(true)) { "Yes" } else { "No" }])
        .add_row(["Labels"]);
    add_labels(&mut table, &pr.labels);
    println!("{table}");
    println!("{}", pr.body.unwrap_or_else(|| "No body provided".into()));
}

fn print_comments(comments: PullRequestComments) {
    if comments.comments.is_empty() {
        println!("No comments.");
    } else {
        println!("{} Comments:", comments.comments.len());
        for c in comments.comments {
            println!("{} ({}) - {}", c.author, c.created_at, c.body);
        }
    }
    if comments.threads.is_empty() {
        println!("No code review threads.");
    } else {
        println!("{} Threads:", comments.threads.len());
        for t in comments.threads {
            let lno = match t.original_line {
                Some(l) => format!("#{}", l),
                None => "".into(),
            };
            println!("File: {}#{lno} - {} comments", t.path, t.comments.len());
            for c in t.comments {
                println!("{} ({}) - {}", c.author, c.created_at, c.body);
            }
        }
    }
}

fn print_review_summary(summary: &ReviewCounts) {
    let mut table = pretty_table(&["PR Reviews", summary.title()]);
    table
        .add_row(["Reviewers", summary.reviewers().join(" ").as_str()])
        .add_row(["Approvals", summary.approvals().to_string().as_str()])
        .add_row(["Changes requested", summary.changes_requested().to_string().as_str()]);
    println!("{table}");
}

fn print_run_status(summary: &CheckRunStatus) {
    let overall = serde_json::to_string(&summary.overall_status()).unwrap();
    println!(
        "Check Run status for commit {} at {}",
        summary.commit_url(),
        summary.committed_at()
    );
    println!("Overall result: {overall}");
    let mut table = pretty_table(&["Name", "Completed At", "Result", "Status", "Required"]);
    for run in summary.checks() {
        let completed_at = run.completed_at.to_string();
        let result = serde_json::to_string(&run.result).unwrap();
        let status = serde_json::to_string(&run.status).unwrap();
        let req = run.is_required.to_string();
        table.add_row([
            run.name.as_str(),
            completed_at.as_str(),
            result.as_str(),
            status.as_str(),
            req.as_str(),
        ]);
    }
    println!("{table}");
}
