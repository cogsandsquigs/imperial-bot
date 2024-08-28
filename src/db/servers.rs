use crate::db::models::*;
use crate::db::{schema, PG_CONNECTION};
use diesel::prelude::*;
use diesel::result::Error;
use poise::serenity_prelude as serenity;
use serenity::{GuildId, RoleId};
use std::ops::DerefMut;

/// Check if a server exists in the database.
pub async fn server_exists(guild_id: GuildId) -> bool {
    use schema::servers::dsl::*;

    match servers
        .find(i64::from(guild_id))
        .first::<Server>(PG_CONNECTION.lock().await.deref_mut())
    {
        Ok(_) => Ok(true),
        Err(diesel::result::Error::NotFound) => Ok(false),
        Err(err) => Err(err),
    }
    .expect("Error checking if server exists") // TODO: Better error handling
}

/// Set the verified role for the server.
pub async fn set_verified_role(guild_id: GuildId, role_id: RoleId) -> Result<(), Error> {
    use schema::servers::dsl::*;

    // Check if the server exists.
    if !server_exists(guild_id).await {
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
pub async fn get_verified_role(guild_id: GuildId) -> Result<Option<RoleId>, Error> {
    use schema::servers::dsl::*;

    // Check if the server exists.
    if !server_exists(guild_id).await {
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
pub async fn get_servers_with_verified_roles() -> Result<Vec<Server>, Error> {
    use schema::servers::dsl::*;

    servers
        .filter(verified_role_id.is_not_null())
        .load(PG_CONNECTION.lock().await.deref_mut())
}
