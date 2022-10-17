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

    let num_subs = rules::load_subscriptions(pubsub.clone()).await?;
    info!("📄 {} Subscriptions loaded", num_subs);

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
    use actix::{Addr, MailboxError};

    use crate::{
        actions::Actions,
        events::{Event, ProgressConstraint, SubscriptionBuilder},
        heuristics::pull_requests::{PullRequestComplexity, PullRequestSize},
        predicates::{PullRequest, PullRequestComment, StatusCheck},
        pub_sub::{PubSubActor, ReplaceRulesMessage, ReplaceSubscriptionsMessage},
        rules::RuleBuilder,
    };

    pub async fn load_rules(pubsub: Addr<PubSubActor>) -> Result<usize, MailboxError> {
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
            RuleBuilder::new("Then action example")
                .when(PullRequest::opened())
                .execute(
                    Actions::closure()
                        .with(|name, event| {
                            let title = (&event.unwrap().pull_request().unwrap().pull_request.title).clone();
                            println!("**** {name}. A pull request was opened: {title} ****");
                        })
                        .build(),
                )
                .then(
                    Actions::closure()
                        .with(|name, event| {
                            let owner = (&event.unwrap().pull_request().unwrap().info.repository.owner.login).clone();
                            println!("**** {name}. And then this happened: {owner} ****");
                        })
                        .build(),
                )
                .submit(),
            RuleBuilder::new("AutoMerge™")
                .when(PullRequest::labeled_with("P-merge"))
                .when(PullRequest::edited())
                .when(PullRequest::approved())
                .when(PullRequestComment::added())
                .when(StatusCheck::suite_success())
                .execute(Actions::auto_merge().with_min_acks(1).auto_merge().build())
                .submit(),
        ];

        let msg = ReplaceRulesMessage { new_rules: rules };

        pubsub.send(msg).await
    }

    pub async fn load_subscriptions(pubsub: Addr<PubSubActor>) -> Result<usize, MailboxError> {
        let subscriptions = vec![
            SubscriptionBuilder::on("Ask for ACKs", Event::AcksNeeded)
                .and(ProgressConstraint::new().max_progress(99))
                .then(Actions::github().add_label("P-acks_required").build())
                .submit(),
            SubscriptionBuilder::on("Ask for reviews", Event::ReviewsNeeded)
                .and(ProgressConstraint::new().max_progress(99))
                .then(Actions::github().add_label("P-reviews_required").build())
                .submit(),
            SubscriptionBuilder::on("Acks achieved", Event::AcksThresholdReached)
                .then(Actions::github().remove_label("P-acks_required").build())
                .submit(),
            SubscriptionBuilder::on("Acks achieved", Event::ReviewsThresholdReached)
                .then(Actions::github().remove_label("P-reviews_required").build())
                .submit(),
        ];

        let msg = ReplaceSubscriptionsMessage {
            new_subscriptions: subscriptions,
        };

        pubsub.send(msg).await
    }
}
