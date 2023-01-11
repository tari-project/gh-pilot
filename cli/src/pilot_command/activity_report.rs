use std::{
    collections::{HashMap, HashSet},
    io::Write,
    path::PathBuf,
};

use github_pilot_api::{
    graphql::org_activity::{IssueActivity, OrgActivity, PullRequestActivity, User},
    models::DateTime,
};
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct ActivityReports {
    reports: HashMap<User, ActivityReport>,
    start: DateTime,
    end: DateTime,
    prs_count: usize,
    issue_count: usize,
    repos: HashSet<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ActivityReport {
    coding_metrics: CodingMetrics,
    engagement_metrics: EngagementMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct CodingMetrics {
    pull_requests_authored: u64,
    pull_requests_merged: u64,
    pull_requests_denied: u64,
    code_volume: CodeVolume,
}

#[derive(Debug, Clone, Default)]
pub struct CodeVolume {
    total_additions: u64,
    total_deletions: u64,
    documentation_volume: u64,
    code_volume: u64,
    files_ignored: HashSet<PathBuf>,
    volume_ignore: u64,
}

#[derive(Debug, Clone, Default)]
pub struct EngagementMetrics {
    issues_created: u64,
    issue_buzz: u64,
    pr_buzz: u64,
    issue_comments: u64,
    code_reviews: u64,
    pr_comments: u64,
}

impl ActivityReport {
    pub fn write_csv<W: Write>(&self, user: &User, f: &mut W) -> Result<(), std::io::Error> {
        let ignored_files = self
            .coding_metrics
            .code_volume
            .files_ignored
            .iter()
            .map(|p| p.to_str().unwrap_or_default())
            .join(":");
        writeln!(
            f,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{ignored_files}",
            user.display_name(),
            self.coding_metrics.pull_requests_authored,
            self.coding_metrics.pull_requests_merged,
            self.coding_metrics.pull_requests_denied,
            self.coding_metrics.code_volume.total_additions,
            self.coding_metrics.code_volume.total_deletions,
            self.coding_metrics.code_volume.documentation_volume,
            self.coding_metrics.code_volume.code_volume,
            self.engagement_metrics.issues_created,
            self.engagement_metrics.code_reviews,
            self.engagement_metrics.issue_comments + self.engagement_metrics.pr_comments,
            self.engagement_metrics.issue_buzz,
            self.engagement_metrics.pr_buzz,
            self.coding_metrics.code_volume.volume_ignore,
        )
    }
}

impl From<OrgActivity> for ActivityReports {
    fn from(activity: OrgActivity) -> Self {
        let mut result = ActivityReports {
            start: DateTime::now(),
            ..Default::default()
        };
        let OrgActivity { issues, pull_requests } = activity;
        result.issue_count = issues.len();
        result.prs_count = pull_requests.len();
        issues.into_iter().for_each(|iss| {
            result.check_time_bounds(&iss.created_at);
            result.add_issue(&iss);
        });
        pull_requests.into_iter().for_each(|pr| {
            result.check_time_bounds(&pr.created_at);
            result.add_pull_request(&pr);
        });
        result
    }
}

impl ActivityReports {
    fn add_issue(&mut self, issue: &IssueActivity) {
        let user = issue.author.clone();
        let report = self.reports.entry(user).or_default();
        // Issues only affect engagement
        report.engagement_metrics.issues_created += 1;
        report.engagement_metrics.issue_buzz += issue.comments.total_comments;
        self.repos.insert(issue.repository.clone());
        issue.comments.comments.iter().for_each(|c| {
            let user = c.author.clone();
            let report = self.reports.entry(user).or_default();
            report.engagement_metrics.issue_comments += 1;
        });
    }

    fn add_pull_request(&mut self, pr: &PullRequestActivity) {
        let user = pr.author.clone();
        let pr_author = self.reports.entry(user).or_default();
        let repo = pr.base_repository.as_deref().unwrap_or("Unknown");
        self.repos.insert(repo.to_string());
        // Coding metrics
        pr_author.coding_metrics.pull_requests_authored += 1;
        if pr.merged {
            pr_author.coding_metrics.pull_requests_merged += 1;
        }
        if pr.closed && !pr.merged {
            pr_author.coding_metrics.pull_requests_denied += 1;
        }
        pr_author.engagement_metrics.pr_buzz += pr.comments.total_comments;
        analyze_code_volume(pr, repo, &mut pr_author.coding_metrics.code_volume);
        self.tally_code_reviews(pr);
        self.tally_pr_comments(pr);
    }

    // Code reviews get added to pr_reviews, and the comments you make in the review get added to pr_comments.
    // These get accrued to the pr _review_ author, not the PR author (they will get buzz).
    fn tally_code_reviews(&mut self, pr: &PullRequestActivity) {
        pr.reviews.reviews.iter().for_each(|r| {
            let user = r.author.clone();
            let review_author = self.reports.entry(user).or_default();
            review_author.engagement_metrics.code_reviews += 1;
            review_author.engagement_metrics.pr_comments += r.total_comments;
        })
    }

    // Attribute comments to comment authors
    fn tally_pr_comments(&mut self, pr: &PullRequestActivity) {
        pr.comments.comments.iter().for_each(|c| {
            let user = c.author.clone();
            let comment_author = self.reports.entry(user).or_default();
            comment_author.engagement_metrics.pr_comments += 1;
        })
    }

    fn check_time_bounds(&mut self, ts: &DateTime) {
        if *ts < self.start {
            self.start = ts.clone();
        }
        if *ts > self.end {
            self.end = ts.clone();
        }
    }

    pub fn write_csv<W: Write>(&self, f: &mut W) -> Result<(), std::io::Error> {
        writeln!(f, "Start date,{},, End date,{}", self.start, self.end)?;
        writeln!(f, "Total PRs, {},,Total Issues,{}", self.prs_count, self.issue_count)?;
        let repo_list = self.repos.iter().join(",");
        writeln!(f, "Repositories:,{repo_list}")?;
        writeln!(f)?;
        writeln!(f, ",Coding Metrics,,,,,,,Engagement Metrics,,,,,,Ignored,,")?;
        writeln!(
            f,
            ",PRs,Merged,Ignored,Additions,Deletions,Docs,Code,Issues,Reviews,Comments,Issue buzz,PR \
             buzz,Ignored,Ignored files"
        )?;
        self.reports
            .iter()
            .sorted_by(|(a, _), (b, _)| a.display_name().cmp(b.display_name()))
            .try_for_each(|(user, report)| report.write_csv(user, f))?;
        Ok(())
    }
}

const CODE_EXTENSIONS: &[&str] = &[
    "rs",
    "js",
    "sh",
    "ts",
    "tsx",
    "py",
    "toml",
    "swift",
    "kt",
    "java",
    "c",
    "h",
    "strings",
    "proto",
    "Dockerfile",
    "yml",
    "toml",
    "sql",
    "css",
    "scss",
    "less",
    "html",
    "lua",
];
const DOC_EXTENSIONS: &[&str] = &["md", "txt", "tex"];
const IGNORED_FILE: &[&str] = &["Cargo.lock", "package-lock.json"];

fn analyze_code_volume(pr: &PullRequestActivity, repo: &str, volume: &mut CodeVolume) {
    volume.total_additions += pr.total_additions;
    volume.total_deletions += pr.total_deletions;
    pr.files
        .files_changed
        .iter()
        .filter(|&f| {
            let filename = f.path.file_name().unwrap_or_default().to_str().unwrap_or_default();
            !IGNORED_FILE.contains(&filename)
        })
        .for_each(|file| {
            let ext = file.path.extension().unwrap_or_default().to_str().unwrap_or_default();
            if CODE_EXTENSIONS.contains(&ext) {
                volume.code_volume += file.additions + file.deletions;
            } else if DOC_EXTENSIONS.contains(&ext) {
                volume.documentation_volume += file.additions + file.deletions;
            } else {
                let mut f = PathBuf::from(repo);
                f.push(&file.path);
                volume.files_ignored.insert(f);
                volume.volume_ignore += file.additions + file.deletions;
            }
        })
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use github_pilot_api::graphql::org_activity::OrgActivity;

    use crate::pilot_command::activity_report::ActivityReports;

    #[test]
    fn convert_from_org_activity() {
        let s = include_str!("../test_data/org_activity.json");
        let activity: OrgActivity = serde_json::from_str(s).unwrap();
        let report = ActivityReports::from(activity);
        let mut f = File::create("org_activity.csv").unwrap();
        report.write_csv(&mut f).unwrap();
    }
}
