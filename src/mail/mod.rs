use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use log::debug;
use std::{
    env,
    sync::{LazyLock, Mutex},
};

/// The global connection to the SMTP server.
/// NOTE: If using `dotenv`, run `dotenv::dotenv().ok();` before using this.
/// TODO: This is bad! Get rid of this!
pub static MAILER: LazyLock<Mutex<SmtpTransport>> = LazyLock::new(|| Mutex::new(establish_smtp()));

fn establish_smtp() -> SmtpTransport {
    let user = env::var("SMTP_USER").expect("SMTP_USER must be set");
    let pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
    let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");

    let creds = Credentials::new(user, pass);

    debug!("Connecting to {}:{} with credentials", host, port);

    SmtpTransport::starttls_relay(&host)
        .unwrap_or_else(|_| panic!("Error connecting to {}:{} with credentials", host, port))
        .credentials(creds)
        .port(port.parse().expect("SMTP_PORT must be a number"))
        .build()
}
