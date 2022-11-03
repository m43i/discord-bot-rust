use crate::lib::{messages::send_dj_message, utils::check_text_channel};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
pub async fn resume(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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

        if let Err(_e) = queue.resume() {
            send_dj_message(&ctx, msg.channel_id, "Nichts zum fortsetzen.".to_string()).await;
        } else {
            send_dj_message(&ctx, msg.channel_id, "Musik ballert wieder.".to_string()).await;
        }
    } else {
        send_dj_message(
            &ctx,
            msg.channel_id,
            "Es gibt nichts zum fortsetzen.".to_string(),
        )
        .await;
    }

    Ok(())
}
