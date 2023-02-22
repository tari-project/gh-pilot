use dotenv::dotenv;
use ghp_server::{cli::handle_command_line_args, config::ServerConfig, server::run_server};
use log::info;

#[actix_web::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    handle_command_line_args();
    let config = ServerConfig::from_env_or_default();

    info!("🚀 Starting server on {}:{}", config.host, config.port);
    match run_server(config).await {
        Ok(_) => println!("Bye!"),
        Err(e) => eprintln!("{e}"),
    }
}
