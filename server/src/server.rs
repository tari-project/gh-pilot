use std::time::Duration;

use actix_web::{http::KeepAlive, web, App, HttpServer};
use actix_web::middleware::Logger;

use crate::{
    config::ServerConfig,
    routes::{github_webhook, health},
};

pub async fn run_server(config: ServerConfig) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t (%D ms) %s %a %{Host}i %U").log_target("ghp_server::logger"))
            .service(health)
            .service(
            web::scope("/github").service(github_webhook),
            )
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(600)))
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
