use dotenv::dotenv;
use serenity::{
    framework::{standard::macros::group, StandardFramework},
    model::prelude::{ChannelType, GuildId, Member, Ready},
    prelude::{Context, EventHandler, GatewayIntents},
    Client,
};
use songbird::SerenityInit;
use std::env;

mod commands;
mod lib;
use crate::commands::add::*;
use crate::commands::deafen::*;
use crate::commands::mute::*;
use crate::commands::pause::*;
use crate::commands::resume::*;
use crate::commands::skip::*;
use crate::commands::stop::*;
use crate::commands::undeafen::*;
use crate::commands::unmute::*;
use crate::{
    commands::play::*,
    lib::{
        messages::send_drink_message,
        utils::{get_channel_ids_from_env, get_voice_members},
    },
};

fn get_reminder_messages() -> Vec<&'static str> {
    let messages = vec![
        "Ein Gläschen in Ehren kann niemand verwehren.",
        "Von der Mitte zur Titte zum Sack, zack, zack!",
        "Euch ist bekannt, was wir bedürfen, wir wollen starke Getränke schlürfen.",
        "Wer Liebe mag und Einigkeit, der trinkt auch mal ne Kleinigkeit.",
        "Essen ist ein Bedürfnis des Magens, Trinken ein Bedürfnis der Seele. Essen ist ein gewöhnliches Handwerk, Trinken eine Kunst.",
        "Zu viel kann man nie trinken, doch trinkt man nie genug!",
        "Es tut mir im Herz so weh, wenn ich vom Glas den Boden seh.",
        "Hau wech die Scheiße!",
        "Du bist dehydriert? Trink Hydration!",
        "N Sekt vielleicht?",
        "Du siehst schlapp aus trink mal lieber was.",
        "Ey Mädels, trinken nicht vergessen.",
        "El Deniz hat bestimmt was in seinem Bauchladen!",
    ];
    messages
}

#[group]
#[commands(play, stop, pause, resume, add, skip, deafen, undeafen, mute, unmute)]
struct General;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache built successfully!");
        let text_channels = get_channel_ids_from_env("DRINK_TEXT_CHANNEL");
        let voice_channels = get_channel_ids_from_env("DRINK_VOICE_CHANNEL");
        let messages = get_reminder_messages();
        tokio::spawn(async move {
            loop {
                let members: Vec<Member> = get_voice_members(&ctx, voice_channels.to_owned())
                    .await
                    .into_iter()
                    .filter(|m| !m.user.bot)
                    .collect();
                if members.len() > 0 {
                    for channel_id in &text_channels {
                        let channel = ctx.cache.channel(*channel_id).unwrap();
                        let channel_id = channel.id();
                        let guild = channel.guild().unwrap();
                        if guild.kind != ChannelType::Voice && guild.is_text_based() {
                            send_drink_message(
                                &ctx,
                                channel_id,
                                messages[rand::random::<usize>() % messages.len()].to_string(),
                                members.to_owned(),
                            )
                            .await;
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        });
    }
}

async fn setup_client() -> Client {
    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");
    client
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut client = setup_client().await;
    client.start().await.unwrap();
}
