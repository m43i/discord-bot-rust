use crate::lib::{messages::send_dj_message, utils::check_text_channel};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
};
use songbird::{input, Call};
use std::io::Error;
use tokio::sync::MutexGuard;

async fn play_music(
    handler: &mut MutexGuard<'_, Call>,
    ctx: &Context,
    msg: &Message,
    url: String,
) -> Result<(), Error> {
    let source = match input::ytdl(&url).await {
        Ok(source) => source,
        Err(why) => {
            println!("Err starting source: {:?}", why);

            send_dj_message(&ctx, msg.channel_id, "Fehler mit ffmpeg".to_string()).await;
            return Ok(());
        }
    };

    handler.enqueue_source(source);

    if handler.queue().len() != 1 {
        let queue = handler.queue();
        queue.modify_queue(|q| q.insert(0, queue.dequeue(queue.len() - 1).unwrap()));
    }

    send_dj_message(
        &ctx,
        msg.channel_id,
        "Musik ballert jetzt auf den Ohren.".to_string(),
    )
    .await;
    Ok(())
}

#[command]
#[only_in(guilds)]
pub async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            send_dj_message(
                &ctx,
                msg.channel_id,
                "Gib eine Video oder Song URL an.".to_string(),
            )
            .await;
            return Ok(());
        }
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
        if let Err(_e) = play_music(&mut handler, &ctx, &msg, url).await {
            send_dj_message(
                &ctx,
                msg.channel_id,
                "Fehler beim Abspielen der Musik.".to_string(),
            )
            .await;
        }
    } else {
        let channel_id = guild
            .voice_states
            .get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);
        let connect_to = match channel_id {
            Some(channel) => channel,
            None => {
                send_dj_message(
                    &ctx,
                    msg.channel_id,
                    "Freundchen du bist in keinem Voice Channel.".to_string(),
                )
                .await;

                return Ok(());
            }
        };
        let (handle_lock, success) = manager.join(guild_id, connect_to).await;
        if let Ok(_channel) = success {
            send_dj_message(
                &ctx,
                msg.channel_id,
                format!("Ich bin jetzt in {}.", connect_to.mention()).to_string(),
            )
            .await;

            let mut handler = handle_lock.lock().await;
            if let Err(_e) = play_music(&mut handler, &ctx, &msg, url).await {
                send_dj_message(
                    &ctx,
                    msg.channel_id,
                    "Fehler beim Abspielen der Musik.".to_string(),
                )
                .await;
            }
        } else {
            send_dj_message(
                &ctx,
                msg.channel_id,
                "Ich habe Probleme beim beitreten.".to_string(),
            )
            .await;
        }
    }

    Ok(())
}
