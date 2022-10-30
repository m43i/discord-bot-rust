use std::env;
use serenity::{model::prelude::{ChannelId, Member, ChannelType}, prelude::Context};

pub fn check_text_channel(channel: ChannelId) -> bool {
    if channel.0 != env::var("DJ_CHANNEL").expect("DJ Channel not set").parse::<u64>().unwrap() {
        return false;
    } else {
        return true;
    }
}

pub async fn get_voice_members(ctx: &Context, channels: Vec<u64>) -> Vec<Member> {
    let mut members: Vec<Member> = vec![];
    for channel in channels {
        let channel = ctx.http.get_channel(channel).await.unwrap();
        let guild = channel.guild().unwrap();
        if guild.kind == ChannelType::Voice {
            let voice_members = guild.members(ctx.cache.as_ref()).await.unwrap();
            for member in voice_members {
                members.push(member);
            }
        }
    }
    members
}

pub fn get_channel_ids_from_env(key: &str) -> Vec<u64> {
    let channels_string = env::var(key).expect("Expected a list of channels in the environment");
    let mut channels = vec![];
    for channel in channels_string.split(",") {
        channels.push(channel.parse::<u64>().unwrap());
    }
    channels
}