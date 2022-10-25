#[cfg(test)]
mod test {
    use crate::{
        actions::Actions,
        heuristics::pull_requests::{PullRequestComplexity, PullRequestSize},
        predicates::{PullRequest, PullRequestComment, StatusCheck},
        rule_set::RuleSet,
        rules::{Rule, RuleBuilder},
    };

    fn rules() -> Vec<Rule> {
        vec![
            RuleBuilder::new("(AutoLabel) Pull request size")
                .when(PullRequest::larger_than(PullRequestSize::Medium))
                .execute(Actions::github().add_label("CR-too_long").build())
                .submit(),
            RuleBuilder::new("(AutoLabel) Pull request complexity")
                .when(PullRequest::more_complex_than(PullRequestComplexity::High))
                .execute(Actions::github().add_label("CR-one_job").build())
                .submit(),
            RuleBuilder::new("(AutoLabel) Pull request justification")
                .when(PullRequest::poor_justification())
                .execute(Actions::github().add_label("CR-insufficient_context").build())
                .submit(),
            RuleBuilder::new("Merge conflict check")
                .when(PullRequest::opened())
                .when(PullRequest::reopened())
                .when(PullRequest::synchronize())
                .when(PullRequest::edited())
                .execute(Actions::github().label_conflicts().build())
                .submit(),
            RuleBuilder::new("Then action example")
                .when(PullRequest::opened())
                .execute(Actions::github().add_label("new PR").build())
                .then(Actions::github().remove_label("old PR").build())
                .submit(),
            RuleBuilder::new("AutoMerge™")
                .when(PullRequest::labeled_with("P-merge"))
                .when(PullRequest::edited())
                .when(PullRequest::approved())
                .when(PullRequestComment::added())
                .when(StatusCheck::suite_success())
                .execute(Actions::auto_merge().with_min_acks(1).auto_merge().build())
                .submit(),
        ]
    }

    #[test]
    fn serialize_github_rule_to_yaml() {
        let rules = RuleSet::from(rules());
        let yaml = serde_yaml::to_string(&rules).unwrap();
        let expected = include_str!("sample_rules_out.yaml");
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_github_rule_to_json() {
        let rules = RuleSet::from(rules());
        let json = serde_json::to_string(&rules).unwrap();
        let expected = include_str!("sample_rules_out.json");
        assert_eq!(json, expected);
    }

    #[test]
    fn deserialize_rules_from_json() {
        let json = include_str!("sample_rules.json");
        let rule_set: RuleSet = serde_json::from_str(json).unwrap();
        assert_eq!(rule_set, RuleSet::from(rules()))
    }

    #[test]
    fn deserialize_rules_from_yaml() {
        let yaml = include_str!("sample_rules.yaml");
        let rule_set: RuleSet = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(rule_set, RuleSet::from(rules()))
    }
}
