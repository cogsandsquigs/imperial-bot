use super::models::*;
use super::{schema, PG_CONNECTION};
use crate::errors::{Error, Result};
use diesel::prelude::*;
use poise::serenity_prelude as serenity;
use serenity::UserId;
use std::ops::DerefMut;

/// Check if a discord user exists in the database.
pub async fn user_exists(user_id: UserId) -> Result<bool> {
    use schema::users::dsl::*;

    match users
        .find(i64::from(user_id))
        .first::<User>(PG_CONNECTION.lock().await.deref_mut())
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(Error::Db(err)),
    }
    // TODO: Better error handling
}

/// Check if a discord user is verified. If the user doesn't exist, return false.
pub async fn is_verified(user_id: UserId) -> Result<bool> {
    use schema::users::dsl::*;

    if !user_exists(user_id).await? {
        return Ok(false);
    }

    // The user must exist to reach here.

    let res = users
        .find(i64::from(user_id))
        .select(state)
        .first::<UserState>(PG_CONNECTION.lock().await.deref_mut())
        .map(|_state| _state == UserState::Verified)?;

    Ok(res)
}

/// Sets a user's state to `state`.
pub async fn set_user_state(user_id: UserId, user_state: UserState) -> Result<()> {
    use schema::users::dsl::*;

    diesel::update(users.find(i64::from(user_id)))
        .set(state.eq(user_state))
        .execute(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(())
}

/// Creates a new user entry given their discord ID, and returns the user object
pub async fn create_user(user_id: UserId) -> Result<User> {
    use schema::users::dsl::*;

    let new_user = NewUser {
        id: i64::from(user_id),
    };

    let u = diesel::insert_into(users)
        .values(&new_user)
        .get_result(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(u)
}

/// Check if this email is already in use by a VERIFIED account.
pub async fn email_exists(email: &str) -> Result<bool> {
    use super::schema::users::dsl::*;

    match users
        .filter(
            state
                .eq(UserState::Verified)
                .and(imperial_email.eq(Some(email))),
        )
        .first::<User>(PG_CONNECTION.lock().await.deref_mut())
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(Error::Db(err)),
    }
}

/// Sets the user's imperial email.
pub async fn set_imperial_email(user_id: UserId, email: String) -> Result<()> {
    use schema::users::dsl::*;

    diesel::update(users.find(i64::from(user_id)))
        .set(imperial_email.eq(Some(email)))
        .execute(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(())
}

/// Inserts an OTP into a user's OTPs.
pub async fn insert_otp(user_id: UserId, otp: i32) -> Result<()> {
    use schema::users::dsl::*;

    diesel::update(users.find(i64::from(user_id)))
        .set(otps.eq(otps.concat(vec![Some(otp)])))
        .execute(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(())
}

/// Check if an OTP exists in a user's OTPs.
pub async fn otp_exists_for_user(user_id: UserId, otp: i32) -> Result<bool> {
    use schema::users::dsl::*;

    match users
        .find(i64::from(user_id))
        .select(otps)
        .first::<Vec<Option<i32>>>(PG_CONNECTION.lock().await.deref_mut())
        .map(|_otps| _otps.contains(&Some(otp)))
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(Error::Db(err)),
    }
}

/// Clear all the user's OTPs.
pub async fn clear_otps(user_id: UserId) -> Result<()> {
    use schema::users::dsl::*;

    diesel::update(users.find(i64::from(user_id)))
        .set(otps.eq::<Vec<i32>>(vec![]))
        .execute(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(())
}

/// Gets all the verified users.
pub async fn get_verified() -> Result<Vec<User>> {
    use schema::users::dsl::*;

    let u = users
        .filter(state.eq(UserState::Verified))
        .load(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(u)
}
