#[cfg(test)]
mod test {
    use crate::{
        actions::Actions,
        heuristics::pull_requests::PullRequestSize,
        predicates::{Predicate, PullRequest, PullRequestComment, StatusCheck},
    };

    #[test]
    fn serialize_github_action_to_json() {
        let action = Actions::github().add_label("JsOn").build();
        let json = serde_json::to_string(&action).unwrap();
        let expected = r#"{"github":{"add_label":"JsOn"}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_github_action_to_yaml() {
        let action = Actions::github().add_label("YAML").build();
        let yaml = serde_yaml::to_string(&action).unwrap();
        let expected = "---\ngithub:\n  add_label: YAML\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_closure_action_to_json() {
        let action = Actions::closure()
            .with(|name, event| {
                println!("{}: {:?}", name, event);
            })
            .build();
        let json = serde_json::to_string(&action).unwrap();
        let expected = r#"{"closure":{}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_closure_action_to_yaml() {
        let action = Actions::closure()
            .with(|name, event| {
                println!("{}: {:?}", name, event);
            })
            .build();
        let yaml = serde_yaml::to_string(&action).unwrap();
        let expected = "---\nclosure: {}\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_null_action_to_json() {
        let action = Actions::null();
        let json = serde_json::to_string(&action).unwrap();
        let expected = r#""none""#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_null_action_to_yaml() {
        let action = Actions::null();
        let yaml = serde_yaml::to_string(&action).unwrap();
        let expected = "---\nnone\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_merge_action_to_json() {
        let action = Actions::auto_merge()
            .auto_merge()
            .with_min_reviews(12)
            .with_min_acks(42)
            .skip_checks()
            .add_ack_pattern("cool")
            .add_ack_pattern("sweet")
            .with_merge_label("🚢")
            .build();
        let json = serde_json::to_string(&action).unwrap();
        let expected = r#"{"merge":{"acks_required":42,"ack_patterns":["cool","sweet"],"reviews_required":12,"all_checks_must_pass":false,"merge_label":"🚢","perform_merge":true}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_merge_action_to_yaml() {
        let action = Actions::auto_merge()
            .auto_merge()
            .skip_checks()
            .add_ack_pattern("cool")
            .add_ack_pattern("sweet")
            .with_merge_label("Meeerrrrge")
            .build();
        let yaml = serde_yaml::to_string(&action).unwrap();
        let expected = r#"---
merge:
  acks_required: 3
  ack_patterns:
    - cool
    - sweet
  reviews_required: 1
  all_checks_must_pass: false
  merge_label: Meeerrrrge
  perform_merge: true
"#;
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_simple_predicate_to_json() {
        let pred = Predicate::PullRequest(PullRequest::opened());
        let json = serde_json::to_string(&pred).unwrap();
        let expected = r#"{"pull_request":"opened"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_simple_predicate_to_yaml() {
        let pred = Predicate::PullRequest(PullRequest::opened());
        let yaml = serde_yaml::to_string(&pred).unwrap();
        let expected = "---\npull_request: opened\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_pr_comment_predicate_to_yaml() {
        let pred = Predicate::PullRequestComment(PullRequestComment::from("James"));
        let yaml = serde_yaml::to_string(&pred).unwrap();
        let expected = "---\npull_request_comment:\n  added: James\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_status_check_predicate_to_yaml() {
        let pred = Predicate::StatusCheck(StatusCheck::suite_success());
        let yaml = serde_yaml::to_string(&pred).unwrap();
        let expected = "---\nstatus_check: check_suite_success\n";
        assert_eq!(yaml, expected);
    }

    #[test]
    fn serialize_complex_predicate_to_json() {
        let pred = Predicate::PullRequest(PullRequest::larger_than(PullRequestSize::Large));
        let json = serde_json::to_string(&pred).unwrap();
        let expected = r#"{"pull_request":{"size_greater_than":"large"}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_predicate_with_param_to_json() {
        let pred = Predicate::PullRequest(PullRequest::assigned_to("Sarah"));
        let json = serde_json::to_string(&pred).unwrap();
        let expected = r#"{"pull_request":{"assigned":"Sarah"}}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn serialize_predicate_with_param_to_yaml() {
        let pred = Predicate::PullRequest(PullRequest::assigned_to("Sarah"));
        let yaml = serde_yaml::to_string(&pred).unwrap();
        let expected = "---\npull_request:\n  assigned: Sarah\n";
        assert_eq!(yaml, expected);
    }
}
