use std::time::Duration;

use actix::Actor;
use actix_web::{http::KeepAlive, middleware::Logger, web, web::Data, App, HttpServer};

use crate::{
    config::ServerConfig,
    pub_sub::PubSubActor,
    routes::{github_webhook, health},
};
use crate::pub_sub::TaskRunner;

pub async fn run_server(config: ServerConfig) -> std::io::Result<()> {
    let task_runner = TaskRunner::default().start();
    let pubsub = Data::new(PubSubActor::new(task_runner.recipient()).start());

    HttpServer::new(move || {
        App::new()
            .app_data(pubsub.clone())
            .wrap(Logger::new("%t (%D ms) %s %a %{Host}i %U").log_target("ghp_server::logger"))
            .service(health)
            .service(web::scope("/github").service(github_webhook))
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(600)))
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
