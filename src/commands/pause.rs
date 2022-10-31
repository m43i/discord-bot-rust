use crate::lib::{messages::send_dj_message, utils::check_text_channel};
use serenity::{prelude::Context, model::prelude::Message, framework::standard::{Args, CommandResult, macros::command}};

#[command]
#[only_in(guilds)]
pub async fn pause(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        if let Err(_e) = queue.pause() {
            send_dj_message(&ctx, msg.channel_id, "Musik kann nicht pausiert werden.".to_string()).await;
        } else {
            send_dj_message(&ctx, msg.channel_id, "Musik pausiert.".to_string()).await;
        }

    } else {
        send_dj_message(&ctx, msg.channel_id, "Es gibt nichts zu pausieren.".to_string()).await;
    }

    Ok(())
}