use crate::lib::{messages::send_dj_message, utils::check_text_channel};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use songbird::input::Restartable;

#[command]
#[only_in(guilds)]
async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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

        // Here, we use lazy restartable sources to make sure that we don't pay
        // for decoding, playback on tracks which aren't actually live yet.
        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                send_dj_message(&ctx, msg.channel_id, "Fehler mit ffmpeg".to_string()).await;

                return Ok(());
            }
        };

        handler.enqueue_source(source.into());

        send_dj_message(
            &ctx,
            msg.channel_id,
            format!(
                "Song zu Queue hinzugefügt, an Position {}",
                handler.queue().len()
            )
            .to_string(),
        )
        .await;
    } else {
        send_dj_message(
            &ctx,
            msg.channel_id,
            "Ich bin in keinem Channel du Clown.".to_string(),
        )
        .await;
    }

    Ok(())
}
