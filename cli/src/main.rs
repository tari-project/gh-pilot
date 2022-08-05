mod cli_def;
mod context;
mod user;

use crate::context::Context;
use crate::user::run_user_cmd;
use clap::Parser;
use cli_def::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();
    let context = Context::mock();
    match &cli.command {
        Some(Commands::User { profile }) => run_user_cmd(&context, profile).await,
        _ => {
            println!("Not yet supported");
            Ok(())
        }
    }
}
