/// Custom result type to wrap both discord results and db results.
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type to wrap both discord errors and db errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Discord error
    #[error("Discord error: {0}")]
    Discord(#[from] poise::serenity_prelude::Error),

    // Database error
    #[error("Database error: {0}")]
    Db(#[from] diesel::result::Error),
}
