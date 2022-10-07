use log::warn;
use regex::Regex;

const DEFAULT_ACKS: usize = 3;
const DEFAULT_REVIEWS: usize = 1;
const DEFAULT_PATTERNS: [&str; 4] = ["^(ut|t)?ACK$", "^LGTM!?$", "^:?\\+1:?$", "^👍$"];
const DEFAULT_LABEL: &str = "P-merge";

#[derive(Clone, Debug)]
pub struct MergeActionParams {
    acks_required: usize,
    ack_patterns: Vec<Regex>,
    reviews_required: usize,
    all_checks_must_pass: bool,
    merge_label: String,
    /// If true, the action will execute the merge automatically IF the auto-merge label is present. If false, the
    /// action will ADD the auto-merge label if all checks pass.
    perform_merge: bool,
}

impl Default for MergeActionParams {
    fn default() -> Self {
        MergeActionParams::builder().build()
    }
}

impl MergeActionParams {
    pub fn builder() -> MergeActionParamsBuilder {
        MergeActionParamsBuilder::new()
    }

    /// Utility function that tests a comment string against the list of ACK strings to determine if it is a valid ack.
    pub fn is_ack(&self, comment: &str) -> bool {
        self.ack_patterns
            .iter()
            .any(|pattern| comment.split('\n').any(|line| pattern.is_match(line)))
    }

    pub fn min_acks_required(&self) -> usize {
        self.acks_required
    }

    pub fn min_reviews_required(&self) -> usize {
        self.reviews_required
    }

    pub fn all_checks_must_pass(&self) -> bool {
        self.all_checks_must_pass
    }

    pub fn merge_label(&self) -> &str {
        self.merge_label.as_str()
    }

    pub fn perform_merge(&self) -> bool {
        self.perform_merge
    }
}

#[derive(Default)]
pub struct MergeActionParamsBuilder {
    acks_required: Option<usize>,
    ack_patterns: Option<Vec<Regex>>,
    reviews_required: Option<usize>,
    all_checks_must_pass: Option<bool>,
    merge_label: Option<String>,
    perform_merge: Option<bool>,
}

impl MergeActionParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the number of ACKs required to merge the PR. These are meant to represent reviews from contributors that
    /// are NOT maintainers of the repository (i.e. they do not have write access to the repository, but have
    /// contributed at least one merged PR to it).
    ///
    /// The [`ack_pattern`]s determine what constitutes a valid ACK.
    ///
    /// The default value is 3.
    pub fn acks_required(mut self, acks_required: usize) -> Self {
        self.acks_required = Some(acks_required);
        self
    }

    /// Adds the given regular expression to the list of patterns that are used to determine if a comment is an ACK.
    /// By default, the following patterns are used:
    /// * `[ut]ACK`
    /// * `LGTM`
    /// * `[:]+1[:]`
    /// * `👍`
    pub fn ack_pattern(mut self, pattern: &str) -> Self {
        match (Regex::new(pattern), &mut self.ack_patterns) {
            (Ok(regex), Some(ref mut ack_patterns)) => ack_patterns.push(regex),
            (Ok(regex), None) => self.ack_patterns = Some(vec![regex]),
            (Err(e), _) => {
                warn!("⏫ Invalid merge action ack pattern: \"{pattern}\": {e} . This pattern will be ignored.")
            },
        };
        self
    }

    /// Sets the minimum number of approved reviews needed to be able to merge the PR. These are "approved" reviews
    /// given through Github and will be subject to the repo's branch protection rules and the like.
    /// The default is 1.
    pub fn reviews_required(mut self, reviews_required: usize) -> Self {
        self.reviews_required = Some(reviews_required);
        self
    }

    /// If true, all checks must pass before the PR can be merged. If false, the PR can be merged even if some checks
    /// fail.
    /// The default is `true`.
    pub fn all_checks_must_pass(mut self, all_checks_must_pass: bool) -> Self {
        self.all_checks_must_pass = Some(all_checks_must_pass);
        self
    }

    /// Define the label that is used to either trigger a merge, or get added to the PR if the PR should be merged.
    /// The specific action depends on the value of [`perform_merge`].
    /// The default is `P-merge`.
    pub fn merge_label(mut self, auto_merge_label: &str) -> Self {
        self.merge_label = Some(auto_merge_label.into());
        self
    }

    /// If true, the action will execute the merge automatically IF the auto-merge label is present. If false, the
    /// action will ADD the auto-merge label if all checks pass.
    /// The default is `false`.
    pub fn perform_merge(mut self, perform_merge: bool) -> Self {
        self.perform_merge = Some(perform_merge);
        self
    }

    /// Builds the [`MergeActionParams`] struct.
    pub fn build(self) -> MergeActionParams {
        MergeActionParams {
            acks_required: self.acks_required.unwrap_or(DEFAULT_ACKS),
            ack_patterns: self.ack_patterns.unwrap_or_else(default_patterns),
            reviews_required: self.reviews_required.unwrap_or(DEFAULT_REVIEWS),
            all_checks_must_pass: self.all_checks_must_pass.unwrap_or(true),
            merge_label: self.merge_label.unwrap_or_else(|| DEFAULT_LABEL.to_string()),
            perform_merge: self.perform_merge.unwrap_or(false),
        }
    }
}

fn default_patterns() -> Vec<Regex> {
    DEFAULT_PATTERNS
        .iter()
        .map(|&pattern| Regex::new(pattern).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_params() {
        let params = MergeActionParams::default();
        assert_eq!(params.acks_required, DEFAULT_ACKS);
        assert_eq!(params.reviews_required, DEFAULT_REVIEWS);
        assert!(params.all_checks_must_pass);
        assert_eq!(params.merge_label, DEFAULT_LABEL);
        assert_eq!(params.perform_merge, false);
    }

    #[test]
    fn default_ack_patterns() {
        let params = MergeActionParams::default();
        assert!(params.is_ack("ACK"));
        assert!(params.is_ack("utACK"));
        // NOT valid acks:
        assert!(!params.is_ack("STACK"));
        assert!(!params.is_ack("RACK up points"));
        assert!(!params.is_ack("This pr makes me ACK"));

        assert!(params.is_ack("LGTM"));
        assert!(params.is_ack("LGTM!"));
        // NOT valid acks:
        assert!(!params.is_ack("LGTM, not!"));
        assert!(!params.is_ack("LGTMBQ-friendly"));

        assert!(params.is_ack(":+1:"));
        assert!(params.is_ack("+1"));
        assert!(params.is_ack("👍"));
        // NOT valid acks:
        assert!(!params.is_ack("👎"));
        assert!(!params.is_ack("well, 1+1=2, so.."));
        assert!(!params.is_ack("👍 or 👎 below"));
        // multiline acks are ok:
        assert!(params.is_ack("Some nits, but looks great.\n👍"));

        // Other invalid ACKS
        assert!(!params.is_ack("\n\n\n\n"));
    }

    #[test]
    fn builder() {
        let params = MergeActionParamsBuilder::new()
            .acks_required(5)
            .ack_pattern("Foo")
            .ack_pattern("b[aA]r")
            .reviews_required(21)
            .all_checks_must_pass(false)
            .merge_label("BAZ")
            .perform_merge(true)
            .build();

        assert_eq!(params.acks_required, 5);
        assert_eq!(params.reviews_required, 21);
        assert_eq!(params.all_checks_must_pass, false);
        assert_eq!(params.merge_label, "BAZ");
        assert!(params.perform_merge);
        // why you should be careful when defining your own ack parameters:
        assert!(params.is_ack("Food"));
        assert!(params.is_ack("My barometer"));
    }
}
