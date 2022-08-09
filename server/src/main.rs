use ghp_server::{config::ServerConfig, server::run_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = ServerConfig::new("127.0.0.1", 8330);
    run_server(config).await
}
