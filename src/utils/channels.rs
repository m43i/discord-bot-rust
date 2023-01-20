use anyhow::Error;
use serenity::{
    json::JsonMap,
    model::prelude::{ChannelType, GuildChannel, Member},
    prelude::Context,
};

/**
 * Get all active voice members
 */
pub async fn get_guild_voice_members(ctx: &Context, id: u64) -> Vec<Member> {
    let mut result: Vec<Member> = vec![];

    let channels = ctx.http.get_channels(id).await;

    if let Err(_) = channels {
        return result;
    }

    let channels = channels.unwrap();

    let guild = ctx.cache.guild(id).unwrap();
    let afk_channel_id = guild.afk_channel_id;

    for channel in channels {
        if channel.kind != ChannelType::Voice {
            continue;
        }

        if let Some(afk_channel_id) = afk_channel_id {
            if channel.id == afk_channel_id {
                continue;
            }
        }

        let members = channel.members(ctx.cache.as_ref()).await;

        if let Err(_) = members {
            continue;
        }

        let members = members.unwrap();

        for member in members {
            result.push(member);
        }
    }

    return result;
}

/**
 * Create a new voice channel
 */
#[allow(dead_code)]
pub async fn create_channel(
    ctx: &Context,
    guild_id: u64,
    category_id: u64,
    name: &str,
    limit: Option<u16>,
) -> Result<GuildChannel, Error> {
    let mut map: JsonMap = JsonMap::new();
    map.insert("name".to_string(), name.to_string().into());
    map.insert("type".to_string(), "VOICE".to_string().into());
    map.insert("parent_id".to_string(), category_id.to_string().into());
    map.insert("limit".to_string(), limit.into());

    let channel = ctx.http.create_channel(guild_id, &map, None).await;

    if let Err(_) = channel {
        return Err(Error::msg("Could not create channel"));
    }

    let channel = channel.unwrap();

    return Ok(channel);
}
