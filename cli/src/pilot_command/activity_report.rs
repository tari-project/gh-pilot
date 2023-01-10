use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use github_pilot_api::{
    graphql::org_activity::{IssueActivity, OrgActivity, PullRequestActivity, User},
    models::DateTime,
};

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
    user: String,
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
    files_ignored: Vec<PathBuf>,
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

impl From<OrgActivity> for ActivityReports {
    fn from(activity: OrgActivity) -> Self {
        let mut result = ActivityReports::default();
        result.start = DateTime::now();
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
        let report = self.reports.entry(user).or_default();
        // Coding metrics
        report.coding_metrics.pull_requests_authored += 1;
        if pr.merged {
            report.coding_metrics.pull_requests_merged += 1;
        }
        if pr.closed && !pr.merged {
            report.coding_metrics.pull_requests_denied += 1;
        }
        analyze_code_volume(&pr, &mut report.coding_metrics.code_volume);
    }

    fn check_time_bounds(&mut self, ts: &DateTime) {
        if *ts < self.start {
            self.start = ts.clone();
        }
        if *ts > self.end {
            self.end = ts.clone();
        }
    }
}

const CODE_EXTENSIONS: &[&str] = &[
    "rs",
    "js",
    "sh",
    "ts",
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
];
const DOC_EXTENSIONS: &[&str] = &["md", "txt", "tex", "html"];
const IGNORED_FILE: &[&str] = &["Cargo.lock", "package-lock.json"];

fn analyze_code_volume(pr: &PullRequestActivity, volume: &mut CodeVolume) {
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
                volume.files_ignored.push(file.path.clone());
                volume.volume_ignore += file.additions + file.deletions;
            }
        })
}

#[cfg(test)]
mod test {
    use github_pilot_api::graphql::org_activity::OrgActivity;

    use crate::pilot_command::activity_report::ActivityReports;

    #[test]
    fn convert_from_org_activity() {
        let s = include_str!("../test_data/org_activity.json");
        let activity: OrgActivity = serde_json::from_str(s).unwrap();
        let report = ActivityReports::from(activity);
        println!("{report:?}");
        panic!("Writing output");
    }
}
