use crate::{lib::{messages::{send_dj_message}, utils::check_text_channel}};
use serenity::{prelude::Context, model::prelude::Message, framework::standard::{Args, CommandResult, macros::command}};
use songbird::input;

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            send_dj_message(&ctx, msg.channel_id, "Gib eine Video oder Song URL an.".to_string()).await;
            return Ok(());
        },
    };

    if !url.starts_with("http") {
        send_dj_message(&ctx, msg.channel_id, "Deine URL ist ungültig.".to_string()).await;
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match input::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                send_dj_message(&ctx, msg.channel_id, "Fehler mit ffmpeg".to_string()).await;

                return Ok(());
            },
        };

        let _ = handler.play_source(source);

        send_dj_message(&ctx, msg.channel_id, "Musik ballert jetzt auf den Ohren.".to_string()).await;
    } else {
        send_dj_message(&ctx, msg.channel_id, "Ich bin noch in keinem Channel mensch.".to_string()).await;
    }

    Ok(())
}