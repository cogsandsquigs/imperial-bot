pub mod models;
pub mod schema;
mod servers;
mod users;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub use servers::*;
pub use users::*;

/// The global connection to the database.
/// NOTE: If using `dotenv`, run `dotenv::dotenv().ok();` before using this.
/// TODO: This is bad! Get rid of this!
pub static PG_CONNECTION: LazyLock<Mutex<PgConnection>> =
    LazyLock::new(|| Mutex::new(establish_connection()));

// This function will establish a connection to the database using the DATABASE_URL environment variable.
// NOTE: If using `dotenv`, run `dotenv::dotenv().ok();` before calling this function.
fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to {}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
