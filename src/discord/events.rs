use super::{Data, Error};
use crate::db::create_user;
use crate::db::get_verified_role;
use crate::db::is_verified;
use crate::db::models::UserState;
use crate::db::set_user_state;
use crate::db::user_exists;
use crate::errors::Result;
use log::info;
use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{Context, CreateMessage, FullEvent};

pub async fn event_handler_wrapper(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> std::result::Result<(), Error> {
    let result = event_handler(ctx, event, _framework, _data).await;

    if let Err(err) = result {
        eprintln!("Error in event handler: {:?}", err);
    }

    Ok(())
}

async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }

        FullEvent::GuildMemberAddition { new_member } => {
            info!(
                "New member {} joined {}",
                new_member.user.name,
                new_member.guild_id.name(ctx).unwrap_or("UNKNOWN".into()),
            );

            let user = &new_member.user;

            // If the user exists, do not insert a new user.
            if user_exists(user.id).await? {
                // If a user with the same discord ID is verified, do not insert a new user.
                // Instead, add their roles.
                if is_verified(user.id).await? {
                    let verified_role = get_verified_role(new_member.guild_id).await?;
                    if let Some(role_id) = verified_role {
                        new_member.add_role(&ctx.http, role_id).await?;
                    }

                    return Ok(());
                }
            }
            // Otherwise, insert a new user.
            else {
                create_user(user.id).await?;
            }

            set_user_state(user.id, UserState::QueryingEmail).await?;

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
        }

        _ => {}
    }
    Ok(())
}
