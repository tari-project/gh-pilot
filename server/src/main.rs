use ghp_server::{config::ServerConfig, server::run_server};

#[actix_web::main]
async fn main() {
    env_logger::init();
    let config = ServerConfig::new("127.0.0.1", 8330);
    match run_server(config).await {
        Ok(_) => println!("Bye!"),
        Err(e) => eprintln!("{e}"),
    }
}
