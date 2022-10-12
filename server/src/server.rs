use std::time::Duration;

use actix::Actor;
use actix_web::{http::KeepAlive, middleware::Logger, web, web::Data, App, HttpServer};
use log::*;

use crate::{
    config::ServerConfig,
    error::ServerError,
    pub_sub::PubSubActor,
    routes::{github_webhook, health},
};

pub async fn run_server(config: ServerConfig) -> Result<(), ServerError> {
    let pubsub = PubSubActor::new().start();

    let num_rules = rules::load_rules(pubsub.clone()).await?;
    info!("📄 {} Rules loaded", num_rules);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pubsub.clone()))
            .wrap(Logger::new("%t (%D ms) %s %a %{Host}i %U").log_target("ghp_server::logger"))
            .service(health)
            .service(web::scope("/github").service(github_webhook))
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(600)))
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
    .map_err(|e| ServerError::Unspecified(e.to_string()))
}

mod rules {
    use actix::Addr;
    use github_pilot_api::webhooks::PullRequestAction;
    use log::info;

    use crate::{
        actions::Actions,
        error::ServerError,
        heuristics::pull_requests::{PullRequestComplexity, PullRequestSize},
        predicates::{PullRequest, PullRequestComment, StatusCheck},
        pub_sub::{PubSubActor, ReplaceRulesMessage},
        rules::RuleBuilder,
    };

    pub async fn load_rules(pubsub: Addr<PubSubActor>) -> Result<usize, ServerError> {
        // This can eventually be replaced with
        // RuleSet::from_json("./config/rules.json")
        // or whatever
        let rules = vec![
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
            {
                let action = Actions::closure()
                    .with(|name, event| {
                        let pr = event.pull_request().unwrap();
                        let label = if let PullRequestAction::Labeled { label } = &pr.action {
                            label.name.as_str()
                        } else {
                            "?"
                        };
                        info!(
                            "{} on PR #{} has been labelled: [{}]. {}",
                            name,
                            pr.number,
                            label,
                            event.summary()
                        )
                    })
                    .build();
                RuleBuilder::new("(Log) Label added")
                    .when(PullRequest::labeled())
                    .execute(action)
                    .submit()
            },
            RuleBuilder::new("AutoMerge™")
                .when(PullRequest::labeled_with("P-merge"))
                .when(PullRequest::edited())
                .when(PullRequest::approved())
                .when(PullRequestComment::added())
                .when(StatusCheck::suite_success())
                .execute(Actions::auto_merge().build())
                .submit(),
        ];

        let msg = ReplaceRulesMessage { new_rules: rules };

        pubsub
            .send(msg)
            .await?
            .map_err(|e| ServerError::RuleConfigurationError(e.to_string()))
    }
}
