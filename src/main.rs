mod db;
mod discord;
mod errors;
mod mail;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env files.

    discord::run().await;
}
