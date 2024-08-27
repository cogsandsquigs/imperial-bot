use super::{Data, Error};
use poise::serenity_prelude as serenity;
use poise::FrameworkContext;
use serenity::{Context, FullEvent};

pub async fn event_handler(
    _ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        FullEvent::GuildMemberAddition { new_member: _ } => {
            // TODO: Handle new member joining
            todo!("Handle new member joining")
        }
        _ => {}
    }
    Ok(())
}
