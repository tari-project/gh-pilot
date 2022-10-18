use std::{cmp::Ordering, fmt::Display};

use crate::events::{BroadcastEvent, EventConstraint};

pub struct Progress {
    pub total: usize,
    pub current: usize,
}

impl Progress {
    pub fn new(current: usize, total: usize) -> Self {
        Self { total, current }
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.current, self.total)
    }
}

#[derive(Default)]
pub struct ProgressConstraint {
    percent_threshold: Option<(usize, Ordering)>,
    count_threshold: Option<(usize, Ordering)>,
    total_threshold: Option<(usize, Ordering)>,
}

impl ProgressConstraint {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn min_progress(mut self, percent: usize) -> Self {
        self.percent_threshold = Some((percent, Ordering::Less));
        self
    }

    pub fn max_progress(mut self, percent: usize) -> Self {
        self.percent_threshold = Some((percent, Ordering::Greater));
        self
    }

    pub fn at_least(mut self, count: usize) -> Self {
        self.count_threshold = Some((count, Ordering::Less));
        self
    }

    pub fn at_most(mut self, count: usize) -> Self {
        self.count_threshold = Some((count, Ordering::Greater));
        self
    }

    pub fn total_at_least(mut self, total: usize) -> Self {
        self.total_threshold = Some((total, Ordering::Less));
        self
    }

    pub fn total_at_most(mut self, total: usize) -> Self {
        self.total_threshold = Some((total, Ordering::Greater));
        self
    }

    fn check_percent(&self, progress: &Progress) -> bool {
        if let Some((threshold, ordering)) = self.percent_threshold {
            let current_percent = if progress.total > 0 {
                (progress.current as f64 / progress.total as f64) * 100.0
            } else {
                100.0
            };
            current_percent.partial_cmp(&(threshold as f64)) != Some(ordering)
        } else {
            true
        }
    }

    fn check_count(&self, progress: &Progress) -> bool {
        if let Some((count, ordering)) = self.count_threshold {
            usize::cmp(&progress.current, &count) != ordering
        } else {
            true
        }
    }

    fn check_total(&self, progress: &Progress) -> bool {
        if let Some((total, ordering)) = self.total_threshold {
            usize::cmp(&progress.total, &total) != ordering
        } else {
            true
        }
    }
}

impl EventConstraint for ProgressConstraint {
    fn matches(&self, event: &BroadcastEvent) -> bool {
        if let BroadcastEvent::ReviewsNeeded(p) | BroadcastEvent::AcksNeeded(p) = event {
            self.check_percent(p) && self.check_count(p) && self.check_total(p)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn reviews(current: usize, total: usize) -> BroadcastEvent {
        let progress = Progress::new(current, total);
        BroadcastEvent::ReviewsNeeded(Box::new(progress))
    }

    fn acks(current: usize, total: usize) -> BroadcastEvent {
        let progress = Progress::new(current, total);
        BroadcastEvent::ReviewsNeeded(Box::new(progress))
    }

    #[test]
    fn min_progress() {
        let constraint = ProgressConstraint::new().min_progress(40);
        assert!(!constraint.matches(&reviews(10, 50)));
        assert!(constraint.matches(&reviews(20, 50)));
        assert!(constraint.matches(&reviews(21, 50)));
        assert!(constraint.matches(&acks(40, 100)));
        assert!(constraint.matches(&acks(80, 100)));
    }

    #[test]
    fn max_progress() {
        let constraint = ProgressConstraint::new().max_progress(40);
        assert!(constraint.matches(&reviews(10, 50)));
        assert!(constraint.matches(&reviews(20, 50)));
        assert!(!constraint.matches(&reviews(21, 50)));
    }

    #[test]
    fn min_count() {
        let constraint = ProgressConstraint::new().at_least(10);
        assert!(!constraint.matches(&reviews(9, 50)));
        assert!(constraint.matches(&reviews(10, 50)));
        assert!(constraint.matches(&reviews(11, 50)));
        assert!(!constraint.matches(&acks(0, 100)));
        assert!(constraint.matches(&acks(100, 100)));
    }

    #[test]
    fn max_count() {
        let constraint = ProgressConstraint::new().at_most(10);
        assert!(constraint.matches(&reviews(9, 50)));
        assert!(constraint.matches(&reviews(10, 50)));
        assert!(!constraint.matches(&reviews(11, 50)));
    }

    #[test]
    fn min_total() {
        let constraint = ProgressConstraint::new().total_at_least(10);
        assert!(!constraint.matches(&reviews(20, 5)));
        assert!(constraint.matches(&reviews(20, 10)));
        assert!(constraint.matches(&reviews(8, 20)));
    }

    #[test]
    fn max_total() {
        let constraint = ProgressConstraint::new().total_at_most(10);
        assert!(constraint.matches(&reviews(20, 5)));
        assert!(constraint.matches(&reviews(20, 10)));
        assert!(!constraint.matches(&reviews(8, 20)));
    }

    #[test]
    fn complex_constraint() {
        let constraint = ProgressConstraint::new()
            .min_progress(25)
            .at_most(10)
            .total_at_least(30);
        assert!(!constraint.matches(&reviews(5, 10)), "Not above 30");
        assert!(!constraint.matches(&reviews(50, 80)), "Count above 10");
        assert!(!constraint.matches(&reviews(8, 40)), "Below 25%");
        assert!(constraint.matches(&reviews(9, 31)));
    }
}
