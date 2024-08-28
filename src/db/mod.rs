pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::*;
use poise::serenity_prelude as serenity;
use serenity::UserId;
use std::env;
use std::ops::DerefMut;
use std::sync::LazyLock;
use tokio::sync::Mutex;

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

/// Check if a discord user exists in the database.
pub async fn user_exists(user_id: UserId) -> bool {
    use schema::users::dsl::*;

    match users
        .find(i64::from(user_id))
        .first::<User>(PG_CONNECTION.lock().await.deref_mut())
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(err),
    }
    .expect("Error checking if user exists") // TODO: Better error handling
}

/// Check if a discord user is verified. If the user doesn't exist, return false.
pub async fn is_verified(user_id: UserId) -> bool {
    use schema::users::dsl::*;

    users
        .find(i64::from(user_id))
        .select(state)
        .first::<UserState>(PG_CONNECTION.lock().await.deref_mut())
        .map(|_state| _state == UserState::Verified)
        .unwrap_or(false) // TODO: Better error handling
}

/// Sets a user's state to `state`.
pub async fn set_user_state(user_id: UserId, user_state: UserState) {
    use schema::users::dsl::*;

    diesel::update(users.find(i64::from(user_id)))
        .set(state.eq(user_state))
        .execute(PG_CONNECTION.lock().await.deref_mut())
        .expect("Error updating user state"); // TODO: Better error handling
}

/// Creates a new user entry given their discord ID, and returns the user object
pub async fn create_user(user_id: UserId) -> User {
    use schema::users::dsl::*;
    let new_user = NewUser {
        id: i64::from(user_id),
    };
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(PG_CONNECTION.lock().await.deref_mut())
        .expect("Error saving new user") // TODO: Better error handling
}

/// Check if this email is already in use by a VERIFIED account.
pub async fn email_exists(email: &str) -> bool {
    use schema::users::dsl::*;
    users
        .filter(
            state
                .eq(UserState::Verified)
                .and(imperial_email.eq(Some(email))),
        )
        .first::<User>(PG_CONNECTION.lock().await.deref_mut())
        .is_ok() // TODO: Better error handling
}

/// Sets the user's imperial email.
pub async fn set_imperial_email(user_id: UserId, email: String) {
    use schema::users::dsl::*;
    diesel::update(users.find(i64::from(user_id)))
        .set(imperial_email.eq(Some(email)))
        .execute(PG_CONNECTION.lock().await.deref_mut())
        .expect("Error updating user email"); // TODO: Better error handling
}

/// Inserts an OTP into a user's OTPs.
pub async fn insert_otp(user_id: UserId, otp: i32) {
    use schema::users::dsl::*;
    diesel::update(users.find(i64::from(user_id)))
        .set(otps.eq(otps.concat(vec![Some(otp)])))
        .execute(PG_CONNECTION.lock().await.deref_mut())
        .expect("Error updating user OTPs"); // TODO: Better error handling
}

/// Check if an OTP exists in a user's OTPs.
pub async fn otp_exists_for_user(user_id: UserId, otp: i32) -> bool {
    use schema::users::dsl::*;
    users
        .find(i64::from(user_id))
        .select(otps)
        .first::<Vec<Option<i32>>>(PG_CONNECTION.lock().await.deref_mut())
        .map(|_otps| _otps.contains(&Some(otp)))
        .unwrap_or(false) // TODO: Better error handling
}

/// Clear all the user's OTPs.
pub async fn clear_otps(user_id: UserId) {
    use schema::users::dsl::*;
    diesel::update(users.find(i64::from(user_id)))
        .set(otps.eq::<Vec<i32>>(vec![]))
        .execute(PG_CONNECTION.lock().await.deref_mut())
        .expect("Error clearing user OTPs"); // TODO: Better error handling
}
