use crate::db::{get_servers_with_verified_roles, get_verified_role, is_verified, models::Server};
use crate::errors::Result;
use poise::serenity_prelude::{CacheHttp, Guild, GuildId, RoleId, UserId};

/// Verify all verified users on a single server.
pub async fn set_verified_role_for_verified_on_single_server<C: CacheHttp>(
    ctx: &C,
    guild_id: GuildId,
) -> Result<()> {
    // Get the role id
    let role_id = if let Some(role_id) = get_verified_role(guild_id).await? {
        role_id
    } else {
        return Ok(());
    };

    let guild = Guild::get(ctx.http(), guild_id).await?;

    let mut guild_members = guild.members(ctx.http(), None, None).await?;

    for member in guild_members.iter_mut() {
        if is_verified(member.user.id).await? {
            member.add_role(ctx.http(), role_id).await?;
        }
    }

    Ok(())
}

/// Verify a user on all servers the user is on.
pub async fn verify_on_all_servers<C: CacheHttp>(ctx: &C, user_id: UserId) -> Result<()> {
    let entries = get_servers_with_verified_roles().await?;

    // TODO: What if the bot has left the server?
    for Server {
        id,
        verified_role_id,
    } in entries
    {
        let guild_id = GuildId::new(id as u64);
        let role_id = RoleId::new(verified_role_id.expect("This should be Some!") as u64);
        let guild = Guild::get(ctx.http(), guild_id).await?;

        let member = guild.member(&ctx.http(), user_id).await?;

        member.add_role(&ctx.http(), role_id).await?;
    }

    Ok(())
}
