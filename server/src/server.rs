use std::{path::Path, time::Duration};

use actix::Actor;
use actix_web::{http::KeepAlive, middleware::Logger, web, web::Data, App, HttpServer};
use log::*;
use notify::{RecursiveMode, Watcher};

use crate::{
    config::ServerConfig,
    error::ServerError,
    file_watch::async_watch,
    load_rules::{load_rules, load_subscriptions},
    pub_sub::PubSubActor,
    routes::{github_webhook, health},
};

pub async fn run_server(config: ServerConfig) -> Result<(), ServerError> {
    let pubsub = PubSubActor::new().start();
    let rule_path = config.rule_set_path.as_str();
    let num_rules = load_rules(pubsub.clone(), rule_path).await?;
    info!("📄 {num_rules} Rules loaded");
    let num_subs = load_subscriptions(pubsub.clone()).await?;
    info!("📄 {num_subs} Subscriptions loaded");

    let mut watcher = async_watch(pubsub.clone(), rule_path);
    if let Some(w) = &mut watcher {
        let path = Path::new(rule_path);
        if let Err(e) = w.watch(path, RecursiveMode::NonRecursive) {
            warn!("Could not watch '{rule_path}'. If you edit the rules, manually restart the server. {e}");
        }
    }

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
