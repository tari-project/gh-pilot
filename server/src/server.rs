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

    let num_rules = rules::load_rules(pubsub.clone(), config.rule_set_path.as_str()).await?;
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
        error::ServerError,
        events::{Event, ProgressConstraint, SubscriptionBuilder},
        pub_sub::{PubSubActor, ReplaceRulesMessage, ReplaceSubscriptionsMessage},
        rule_set::RuleSet,
    };

    pub async fn load_rules(pubsub: Addr<PubSubActor>, rules_path: &str) -> Result<usize, ServerError> {
        let rules = RuleSet::from_yaml(rules_path).map_err(|e| ServerError::RuleConfigurationError(e.to_string()))?;

        let msg = ReplaceRulesMessage {
            new_rules: rules.to_rules(),
        };

        let result = pubsub.send(msg).await?;
        Ok(result)
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
