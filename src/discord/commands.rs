use super::{
    roles::{set_verified_role_for_verified_on_single_server, verify_on_all_servers},
    Context, Error,
};
use crate::db::models::*;
use crate::db::{
    clear_otps, create_user, email_exists, insert_otp, is_verified, otp_exists_for_user,
    set_imperial_email, set_user_state, set_verified_role as set_verified_role_db, user_exists,
};
use crate::mail::MAILER;
use lettre::message::header::ContentType;
use lettre::{Message, Transport};
use log::info;
use poise::serenity_prelude::{self as serenity, CreateMessage};
use rand::Rng;
use std::env;
use std::ops::DerefMut;

/// Starts the process of verifying a user.
#[poise::command(slash_command, guild_only)]
pub async fn verify(
    ctx: Context<'_>,
    #[description = "User to verify"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.unwrap_or_else(|| ctx.author().clone());

    // If the user exists, do not insert a new user.
    if user_exists(user.id)
        .await
        .expect("Error checking if user exists")
    {
        // If a user with the same discord ID is verified, do not insert a new user.
        if is_verified(user.id)
            .await
            .expect("Error checking if user is verified")
        {
            ctx.say("User is already verified!").await?;
            return Ok(());
        }

        ctx.say("User verification process restarted!").await?;
    }
    // Otherwise, insert a new user.
    else {
        create_user(user.id).await.expect("Error creating user");
        ctx.say("User verification process started!").await?;
    }

    set_user_state(user.id, UserState::QueryingEmail)
        .await
        .expect("Error setting user state");

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
#[poise::command(slash_command, dm_only)]
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
    if email_exists(email)
        .await
        .expect("Error checking if email exists")
    {
        ctx.say("Sorry, the email you provided is already in use. Please provide a unique Imperial email.")
            .await?;
        return Ok(());
    }

    let otp = rand::thread_rng().gen_range(100000..=99999999);

    insert_otp(user.id, otp).await.expect("Error inserting OTP");

    let email_msg = Message::builder()
        .from(
            env::var("SMTP_FROM")
                .expect("SMTP_FROM is required!")
                .parse()
                .unwrap(),
        )
        .to(email.parse().unwrap()) // TODO: Error handling
        .subject("Verify your Imperial Email")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "Hello, {}! Your secret password is {}",
            user.name, otp
        ))
        .unwrap();

    MAILER.lock().unwrap().deref_mut().send(&email_msg).unwrap();

    set_user_state(user.id, UserState::QueryingOTP)
        .await
        .expect("Error setting user state");
    set_imperial_email(user.id, email.to_string())
        .await
        .expect("Error setting imperial email");

    ctx.say(
        r"Thank you!
        Now, run the `/otp` command with the secret passcode sent to your email.",
    )
    .await?;

    Ok(())
}

/// Verifies your email with your secret passcode.
#[poise::command(slash_command, dm_only)]
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

    if otp_exists_for_user(user.id, otp)
        .await
        .expect("Error checking if OTP exists")
    {
        clear_otps(user.id).await.expect("Error clearing OTPs");
        set_user_state(user.id, UserState::Verified)
            .await
            .expect("Error setting user state");
        verify_on_all_servers(&ctx, user.id).await?;

        info!("Verified user {}", user.name);

        ctx.say("Congratulations! You've been verified!").await?;
    } else {
        // Keep them in the same state, so they can try again.
        ctx.say("Sorry, the secret passcode you provided is incorrect. Please provide the correct secret passcode.").await?;
    }

    Ok(())
}

/// Sets the server's verified user role.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn set_verified_role(
    ctx: Context<'_>,
    #[description = "Role to set"] role: serenity::Role,
) -> Result<(), Error> {
    set_verified_role_db(ctx.guild_id().unwrap(), role.id)
        .await
        .unwrap();

    set_verified_role_for_verified_on_single_server(&ctx, ctx.guild_id().unwrap())
        .await
        .expect("Error setting verified role"); // TODO: Better error handling

    ctx.say(format!("Verified role set to `{}`!", role.name))
        .await?;

    Ok(())
}
