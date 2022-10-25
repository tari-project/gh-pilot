use github_pilot_api::models::PullRequest;
use log::trace;
use serde::{Deserialize, Serialize};

pub struct PullRequestHeuristics<'pr> {
    pr: &'pr PullRequest,
}

impl<'pr> PullRequestHeuristics<'pr> {
    pub fn new(pr: &'pr PullRequest) -> Self {
        Self { pr }
    }

    /// A heuristic to indicate the size of a PR. Currently, the only metrics used are
    /// * additions
    /// * deletions
    pub fn size(&self) -> PullRequestSize {
        let additions = self.pr.additions.unwrap_or(0);
        let total = self.total_changes();

        let size = if total < 5 {
            PullRequestSize::Tiny
        } else if additions < 25 || total < 100 {
            PullRequestSize::Small
        } else if additions < 500 || total < 1000 {
            PullRequestSize::Medium
        } else if additions < 800 || total < 2000 {
            PullRequestSize::Large
        } else {
            PullRequestSize::Huge
        };
        trace!("🐙⛰ PR size heuristic: {:?}", size);
        size
    }

    pub fn total_changes(&self) -> usize {
        let additions = self.pr.additions.unwrap_or(0);
        let deletions = self.pr.deletions.unwrap_or(0);
        additions + deletions
    }

    /// A heuristic to indicate the complexity of a PR. Currently, the metrics used to determine complexity are
    /// * PR size heuristic
    /// * Number of files changed
    pub fn complexity(&self) -> PullRequestComplexity {
        let additions = self.pr.additions.unwrap_or(0);
        let deletions = self.pr.deletions.unwrap_or(0);
        let commit_count = self.pr.commits.unwrap_or(2) as f64;
        let files_changed = self.pr.changed_files.unwrap_or(1) as f64;
        let complexity = complexity_heuristic(additions, deletions, commit_count, files_changed);
        trace!("🐙⛰ PR complexity heuristic: {:?}", complexity);
        complexity
    }

    /// Estimates whether the PR body has sufficient context to describe the changes in the PR.
    pub fn has_sufficient_context(&self) -> bool {
        let changes = self.total_changes();
        let body_length = self.pr.body.as_ref().map(|s| s.len()).unwrap_or(0);
        // Want at least 100 characters,
        // and at least 1 character per 10 lines of code changed
        // but any message more than 10 lines (80 words or so), or 400 characters is probably fine
        if body_length > 500 {
            return true;
        }
        body_length >= 100.max(changes / 10)
    }
}

fn complexity_heuristic(
    additions: usize,
    deletions: usize,
    commit_count: f64,
    files_changed: f64,
) -> PullRequestComplexity {
    let total = (additions + deletions) as f64;

    // If |additions - deletions| is large, the there is mostly new code, or mostly removing code, so complexity
    // is lower.
    // But the total number of changes is important too.
    let size_complexity = total.powf(1.25) / (additions.abs_diff(deletions).max(10) as f64).sqrt();
    let complexity_score = 3.0 * commit_count + 1.5 * files_changed + size_complexity;
    trace!("🐙⛰ Complexity score: {}", complexity_score);
    match complexity_score as usize {
        0..=20 => PullRequestComplexity::Low,
        21..=250 => PullRequestComplexity::Medium,
        251..=3000 => PullRequestComplexity::High,
        _ => PullRequestComplexity::VeryHigh,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestComplexity {
    Low,
    Medium,
    High,
    VeryHigh,
}

#[cfg(test)]
mod test {
    use super::PullRequestComplexity;

    #[test]
    fn complexity_heuristic() {
        let complexity_score = super::complexity_heuristic(10, 0, 1.0, 1.0);
        assert_eq!(complexity_score, PullRequestComplexity::Low);

        let complexity_score = super::complexity_heuristic(120, 60, 2.0, 5.0);
        assert_eq!(complexity_score, PullRequestComplexity::Medium);

        let complexity_score = super::complexity_heuristic(250, 120, 1.0, 15.0);
        assert_eq!(complexity_score, PullRequestComplexity::Medium);

        let complexity_score = super::complexity_heuristic(1000, 0, 3.0, 5.0);
        assert_eq!(complexity_score, PullRequestComplexity::Medium);

        let complexity_score = super::complexity_heuristic(1000, 400, 5.0, 30.0);
        assert_eq!(complexity_score, PullRequestComplexity::High);

        let complexity_score = super::complexity_heuristic(700, 700, 7.0, 50.0);
        assert_eq!(complexity_score, PullRequestComplexity::High);

        let complexity_score = super::complexity_heuristic(1200, 1200, 3.0, 40.0);
        assert_eq!(complexity_score, PullRequestComplexity::VeryHigh);

        let complexity_score = super::complexity_heuristic(16000, 0, 1.0, 1.0);
        assert_eq!(complexity_score, PullRequestComplexity::High);
    }
}
