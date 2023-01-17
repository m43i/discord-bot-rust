use serenity::{
    model::prelude::{ChannelType, Member},
    prelude::Context,
};

/**
 * Get all active voice members
 */
pub async fn get_voice_members(ctx: &Context, id: u64) -> Vec<Member> {
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
