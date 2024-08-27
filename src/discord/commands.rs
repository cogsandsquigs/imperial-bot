use super::{Context, Error};
use crate::db::{
    clear_otps, create_user, insert_otp, is_verified, otp_exists_for_user, set_imperial_email,
    set_user_state,
};
use crate::db::{models::*, user_exists};
use poise::serenity_prelude::{self as serenity, CreateMessage};

/// Starts the process of verifying a user.
#[poise::command(slash_command)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "User to verify"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());

    // If the user exists, do not insert a new user.
    if user_exists(user.id).await {
        // If a user with the same discord ID is verified, do not insert a new user.
        if is_verified(user.id).await {
            ctx.say("User is already verified!").await?;
            return Ok(());
        }

        ctx.say("User verification process restarted!").await?;
    }
    // Otherwise, insert a new user.
    else {
        create_user(user.id).await;
        ctx.say("User verification process started!").await?;
    }

    // TODO: Start verification process

    set_user_state(user.id, UserState::QueryingEmail).await;

    // Ask for their Imperial email.
    user.dm(
        ctx,
        CreateMessage::new().content(
            r"Hello! It looks like you've joined a server for Imperial students. 
                This server requires an extra step of verification before you can join. 
                Please provide your Imperial email via the `/set_email` command.",
        ),
    )
    .await?;

    Ok(())
}

/// Sets your imperial email.
#[poise::command(slash_command)]
pub async fn set_email(
    ctx: Context<'_>,
    #[description = "Email to set"] email: String,
) -> Result<(), Error> {
    let user = ctx.author();

    // Preprocess the email, and check if it's valid.
    let email = email.trim();

    if !email.ends_with("@imperial.ac.uk") {
        ctx.say(
           "Sorry, the email you provided is not an Imperial email. Please provide an Imperial email."
        )
        .await?;
        return Ok(());
    }

    // Make sure the email is unique.
    // TODO: Check if the email is unique in the database

    // TODO: Generate an OTP randomly from 100000 to 99999999 (6-8 digits)
    let otp = 123456;

    insert_otp(user.id, otp).await;

    // TODO: Email the user an OTP

    set_user_state(user.id, UserState::QueryingOTP).await;
    set_imperial_email(user.id, email.to_string()).await;

    ctx.say("Thank you! Now, run the /otp command with the secret passcode sent to your email.")
        .await?;

    Ok(())
}

/// Verifies your email with your secret passcode.
#[poise::command(slash_command)]
pub async fn otp(
    ctx: Context<'_>,
    #[description = "The secret passcode to set"] otp: i32,
) -> Result<(), Error> {
    let user = ctx.author();

    // Check if the OTP is valid.
    if !(100000..=99999999).contains(&otp) {
        ctx
            .say("Sorry, the secret passcode you provided is invalid. Please provide a valid secret passcode.")
            .await?;

        return Ok(());
    };

    // Check if the OTP is correct.
    let is_verified = otp_exists_for_user(user.id, otp).await;

    if is_verified {
        clear_otps(user.id).await;
        set_user_state(user.id, UserState::Verified).await;
        ctx.say("Congratulations! You've been verified!").await?;
    } else {
        // Keep them in the same state, so they can try again.
        ctx.say("Sorry, the secret passcode you provided is incorrect. Please provide the correct secret passcode.").await?;
    }

    Ok(())
}
