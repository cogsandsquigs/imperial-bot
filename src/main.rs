mod db;
mod discord;
mod errors;
mod mail;

use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::info;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env files.

    Builder::from_env(Env::default().filter("LOG_LEVEL")).init();

    info!("Starting up...");

    discord::run().await;
}
