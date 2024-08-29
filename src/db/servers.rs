use crate::db::models::*;
use crate::db::{schema, PG_CONNECTION};
use crate::errors::{Error, Result};
use diesel::prelude::*;
use poise::serenity_prelude as serenity;
use serenity::{GuildId, RoleId};
use std::ops::DerefMut;

/// Check if a server exists in the database.
pub async fn server_exists(guild_id: GuildId) -> Result<bool> {
    use schema::servers::dsl::*;

    match servers
        .find(i64::from(guild_id))
        .first::<Server>(PG_CONNECTION.lock().await.deref_mut())
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(Error::Db(err)),
    }
}

/// Set the verified role for the server.
pub async fn set_verified_role(guild_id: GuildId, role_id: RoleId) -> Result<()> {
    use schema::servers::dsl::*;

    // Check if the server exists.
    if !server_exists(guild_id).await? {
        // If it doesn't exist, create it.
        diesel::insert_into(servers)
            .values(&NewServer {
                id: i64::from(guild_id),
            })
            .execute(PG_CONNECTION.lock().await.deref_mut())?;
    }

    // Update the verified role.
    diesel::update(servers.find(i64::from(guild_id)))
        .set(verified_role_id.eq(Some(i64::from(role_id))))
        .execute(PG_CONNECTION.lock().await.deref_mut())?;
    Ok(())
}

/// Get the verified role for the server.
pub async fn get_verified_role(guild_id: GuildId) -> Result<Option<RoleId>> {
    use schema::servers::dsl::*;

    // Check if the server exists.
    if !server_exists(guild_id).await? {
        // If it doesn't exist, return None.
        return Ok(None);
    }

    // Get the verified role.
    let server: Server = servers
        .find(i64::from(guild_id))
        .first(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(server.verified_role_id.map(|_id| RoleId::new(_id as u64)))
}

/// Get all the servers with verified roles.
pub async fn get_servers_with_verified_roles() -> Result<Vec<Server>> {
    use schema::servers::dsl::*;

    let res = servers
        .filter(verified_role_id.is_not_null())
        .load(PG_CONNECTION.lock().await.deref_mut())?;

    Ok(res)
}
