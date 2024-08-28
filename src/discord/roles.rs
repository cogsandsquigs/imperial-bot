use crate::db::{get_servers_with_verified_roles, get_verified_role, is_verified, models::Server};
use poise::serenity_prelude::{CacheHttp, Error, Guild, GuildId, RoleId, UserId};

/// Verify all verified users on a single server.
pub async fn set_verified_role_for_verified_on_single_server<C: CacheHttp>(
    ctx: &C,
    guild_id: GuildId,
) -> Result<(), Error> {
    // Get the role id
    let role_id = if let Some(role_id) = get_verified_role(guild_id)
        .await
        .expect("Error getting role")
    // TODO: Better error handling
    {
        role_id
    } else {
        return Ok(());
    };

    let guild = Guild::get(ctx.http(), guild_id)
        .await
        .expect("Error getting guild"); // TODO: Better error handling

    let mut guild_members = guild
        .members(ctx.http(), None, None)
        .await
        .expect("Error getting members"); // TODO: Better error handling

    for member in guild_members.iter_mut() {
        if is_verified(member.user.id).await {
            member
                .add_role(ctx.http(), role_id)
                .await
                .expect("Error adding role to user"); // TODO: Better error handling
        }
    }

    Ok(())
}

/// Verify a user on all servers the user is on.
pub async fn verify_on_all_servers<C: CacheHttp>(ctx: &C, user_id: UserId) -> Result<(), Error> {
    let entries = get_servers_with_verified_roles()
        .await
        .expect("Error getting servers"); // TODO: Better error handling

    // TODO: What if the bot has left the server?
    for Server {
        id,
        verified_role_id,
    } in entries
    {
        let guild_id = GuildId::new(id as u64);
        let role_id = RoleId::new(verified_role_id.expect("This should be Some!") as u64);

        let guild = Guild::get(ctx.http(), guild_id)
            .await
            .expect("Error getting guild"); // TODO: Better error handling

        let member = guild
            .member(&ctx.http(), user_id)
            .await
            .expect("Error getting member"); // TODO: Better error handling

        member
            .add_role(&ctx.http(), role_id)
            .await
            .expect("Error adding role to user"); // TODO: Better error handling
    }

    Ok(())
}
