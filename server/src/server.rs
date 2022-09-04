use std::time::Duration;

use actix_web::{guard, http::KeepAlive, web, App, HttpServer};

use crate::{
    config::ServerConfig,
    routes::{github_webhook, health},
};

pub async fn run_server(config: ServerConfig) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(
            web::scope("/github")
                .guard(guard::Header("Host", "www.github.com"))
                .service(github_webhook),
            )
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(600)))
    .bind((config.host.as_str(), config.port))?
    .run()
    .await
}
