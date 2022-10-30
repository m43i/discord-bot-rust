use serenity::{framework::standard::{macros::command, CommandResult}, prelude::Context, model::prelude::Message};

use crate::lib::{utils::check_text_channel, messages::send_dj_message};

#[command]
#[only_in(guilds)]
pub async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = match manager.get(guild_id) {
        Some(handler) => handler,
        None => {
            send_dj_message(&ctx, msg.channel_id, "Ich bin in keinem Channel, also kann ich auch nicht stumm sein.".to_string()).await;
            return Ok(());
        },
    };

    let mut handler = handler_lock.lock().await;

    if handler.is_mute() {
        send_dj_message(&ctx, msg.channel_id, "Ich bin doch schon stumm.".to_string()).await;
    } else {
        if let Err(e) = handler.mute(true).await {
            send_dj_message(&ctx, msg.channel_id, format!("Fehler: {:?}", e).to_string()).await;
        }

        send_dj_message(&ctx, msg.channel_id, "Okay ich bin leise.".to_string()).await;
    }

    Ok(())
}