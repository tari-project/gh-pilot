use actix_web::{App, HttpServer};
use crate::config::ServerConfig;
use crate::routes::{health};

pub async fn run_server(config: ServerConfig) -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(health)
    )
        .bind((config.host.as_str(), config.port))?
        .run()
        .await
}