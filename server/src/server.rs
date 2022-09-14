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
    info!("{} Rules loaded", num_rules);

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
    use ghp_api::webhooks::PullRequestAction;

    use crate::{
        actions::Actions,
        error::ServerError,
        predicates::PullRequest,
        pub_sub::{PubSubActor, ReplaceRulesMessage},
        rules::RuleBuilder,
    };

    pub async fn load_rules(pubsub: Addr<PubSubActor>) -> Result<usize, ServerError> {
        // This can eventually be replaced with
        // RuleSet::from_json("./config/rules.json")
        // or whatever
        let rules = vec![
            {
                let action = Actions::github().add_label("P-conflicts").build();
                RuleBuilder::new("AutoLabel - Merge conflicts")
                    .when(PullRequest::opened())
                    .when(PullRequest::synchronize())
                    .execute(action)
                    .submit()
            },
            {
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
                RuleBuilder::new("When label removed")
                    .when(PullRequest::unlabeled())
                    .execute(action)
                    .submit()
            },
        ];

        let msg = ReplaceRulesMessage { new_rules: rules };

        pubsub
            .send(msg)
            .await?
            .map_err(|e| ServerError::RuleConfigurationError(e.to_string()))
    }
}
