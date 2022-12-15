use github_pilot_api::{
    graphql::{review_counts::ReviewCounts, CheckRunStatus, Comment, PullRequestComments},
    models::PullRequest,
    models_plus::MergeParameters,
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

use crate::pretty_print::{add_labels, pretty_table};

#[derive(Debug)]
pub enum PrCmd {
    /// Fetches a PR
    Fetch(IssueId),
    /// Adds a label to a PR
    AddLabel(IssueId, String),
    /// Removes a label from a PR
    RemoveLabel(IssueId, String),
    /// Retrieve the comments and review comment threads for a PR
    Comments(IssueId),
    /// Try to merge the PR
    Merge(IssueId, MergeParameters),
    /// Fetch review statistics
    Reviews(IssueId),
    /// Fetch last check run status (The result of the checks that are configured to run after each PR change).
    Check(IssueId),
    /// Add a comment to the PR
    AddComment(IssueId, String),
}

impl PrCmd {
    pub async fn execute(self, provider: &GithubProvider) -> Result<(), String> {
        match self {
            PrCmd::Fetch(ref id) => fetch_pr(provider, id).await,
            PrCmd::AddLabel(ref id, l) => add_label(provider, id, l.as_str()).await,
            PrCmd::RemoveLabel(ref id, l) => Self::remove_label(provider, id, l.as_str()).await,
            PrCmd::Comments(ref id) => fetch_comments(provider, id).await,
            PrCmd::Merge(ref id, params) => merge_pull_request(provider, id, params).await,
            PrCmd::Reviews(ref id) => fetch_review_summary(provider, id).await,
            PrCmd::Check(ref id) => fetch_last_check_run(provider, id).await,
            PrCmd::AddComment(ref id, c) => add_comment(provider, id, c.as_str()).await,
        }
    }

    async fn remove_label(provider: &GithubProvider, id: &IssueId, label: &str) -> Result<(), String> {
        match provider.remove_label(id, label, false).await {
            Ok(true) => Ok(info!("⏩ '{label}' removed from PR {id}")),
            Ok(false) => Ok(info!("⏩ '{label}' was not present on PR {id}")),
            Err(e) => Err(format!("⏩ Error removing label from PR: {e}")),
        }
    }
}

async fn add_label(provider: &GithubProvider, id: &IssueId, label: &str) -> Result<(), String> {
    match provider.add_label(id, label).await {
        Ok(labels) => {
            let labels = labels.into_iter().map(|l| l.name).collect::<Vec<String>>().join(", ");
            info!("🏷 Labels added to {id}: {labels}");
            Ok(())
        },
        Err(e) => Err(format!("Error adding {label} to {id}: {e}")),
    }
}

async fn fetch_pr(provider: &dyn PullRequestProvider, id: &IssueId) -> Result<(), String> {
    match provider.fetch_pull_request(id).await {
        Ok(pr) => {
            pretty_print(pr);
            Ok(())
        },
        Err(e) => Err(format!("⏩ Error fetching PR {id}: {e}")),
    }
}

async fn fetch_comments(provider: &dyn PullRequestCommentsProvider, id: &IssueId) -> Result<(), String> {
    match provider.fetch_pull_request_comments(id).await {
        Ok(comments) => {
            print_pr_comments(comments);
            Ok(())
        },
        Err(e) => Err(format!("⏩ Error fetching PR {id} comments: {e}")),
    }
}

async fn add_comment(provider: &GithubProvider, id: &IssueId, comment: &str) -> Result<(), String> {
    match provider.add_comment(id, comment).await {
        Ok(c) => {
            info!("⏩📝 Comment added to {id}: {}", c.url);
            Ok(())
        },
        Err(e) => Err(format!("Error adding {comment} to {id}: {e}")),
    }
}

async fn fetch_review_summary(provider: &dyn PullRequestReviewSummary, id: &IssueId) -> Result<(), String> {
    match provider.fetch_review_summary(id).await {
        Ok(summary) => {
            print_review_summary(&summary);
            Ok(())
        },
        Err(e) => Err(format!("⏩👀 Error fetching PR {id} review summary: {e}")),
    }
}

async fn fetch_last_check_run(provider: &dyn CheckRunStatusProvider, id: &IssueId) -> Result<(), String> {
    match provider.fetch_check_run(id).await {
        Ok(run_status) => {
            print_run_status(&run_status);
            Ok(())
        },
        Err(e) => Err(format!("⏩✅ Error fetching PR {id} review summary: {e}")),
    }
}

async fn merge_pull_request(
    provider: &dyn PullRequestProvider,
    id: &IssueId,
    params: MergeParameters,
) -> Result<(), String> {
    match provider.merge_pull_request(id, params).await {
        Ok(r) => {
            if r.merged {
                info!("⏩ PR {id} was merged successfully. {}", r.message);
            } else {
                warn!("⏩ The PR {id} was NOT merged. {}", r.message);
            }
            Ok(())
        },
        Err(e) => Err(format!("⏩ Error merging PR {id}: {e}")),
    }
}

//-----------------------------------------------     Output formatting     -------------------------------------------

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

fn print_comments(comments: &[Comment]) {
    if comments.is_empty() {
        println!("No comments.");
    } else {
        println!("{} Comments:", comments.len());
        for c in comments {
            println!("{} ({}) - {}", c.author, c.created_at, c.body);
        }
    }
}

fn print_pr_comments(comments: PullRequestComments) {
    print_comments(&comments.comments);
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
        let req = run.is_required.to_string();
        table.add_row([run.name.as_str(), completed_at.as_str(), result.as_str(), req.as_str()]);
    }
    println!("{table}");
}
