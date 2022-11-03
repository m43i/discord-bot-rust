use crate::lib::{messages::send_dj_message, utils::check_text_channel};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
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
        let mut handler = handler_lock.lock().await;
        if let Err(e) = handler.deafen(false).await {
            send_dj_message(&ctx, msg.channel_id, format!("Failed: {:?}", e).to_string()).await;
        }

        send_dj_message(&ctx, msg.channel_id, "Ich höre wieder was.".to_string()).await;
    } else {
        send_dj_message(
            &ctx,
            msg.channel_id,
            "Du kannst mich nicht taub machen.".to_string(),
        )
        .await;
    }

    Ok(())
}
