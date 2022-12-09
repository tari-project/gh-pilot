use dotenv::dotenv;
use ghp_server::{config::ServerConfig, server::run_server};
use log::info;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let config = ServerConfig::new("127.0.0.1", 8330);
    info!("🚀 Starting server on {}:{}", config.host, config.port);
    match run_server(config).await {
        Ok(_) => println!("Bye!"),
        Err(e) => eprintln!("{e}"),
    }
}
