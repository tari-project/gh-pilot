# Github Pilot

![Logo](./gh_pilot_logo.jpg)

_Make your Github CI processes fly._

# Github pilot server

The primary point of the GH Pilot server is to define rules using a great, fluid API.

Rules are collections of _predicates_ that trigger one or more _actions_ when the predicate conditions match against 
incoming Github events.

```rust
  RuleBuilder::new("AutoLabel - Demo (add bar)")
      .when(PullRequest::labeled_with("T-foo"))
      .execute(Actions::github().add_label("T-bar").build())
      .submit(),
```

You can essentially perform any action when a rule's conditions are met. Even arbitrary code:

```rust
    let action = Actions::closure()
        .with(|_name, event| {
            let pr = event.pull_request().unwrap();
            let label = if let PullRequestAction::Unlabeled { label } = &pr.action {
                label.name.as_str()
            } else {
                "No label name found"
            };
            println!("PR #{} has had label [{}] removed.", pr.number, label)
        })
        .build();
    RuleBuilder::new("Print a message when a label is removed")
        .when(PullRequest::unlabeled())
        .execute(action)
        .submit()
```

Github pilot is far more flexible and easier to manage than Github Actions.

* Automatically approve a PR if _n_ "+1" comments are received from whitelisted users,
* Automatically label PRs based on heuristics,
* Trigger github actions based on fine-grained event predicates,
* and much more

# Github pilot CLI

The CLI tool is a handy partner client app to Github pilot. You can use it to do many of the things you do in the 
Github website, without having to leave your terminal.

* Add a label to an issue

```bash
ghp-cli issue -n 123 add-label "T-foo"
```

..and so on. See `ghp-cli --help` for more.