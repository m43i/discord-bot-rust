use serenity::{framework::standard::{macros::command, CommandResult, Args}, prelude::Context, model::prelude::Message};
use crate::lib::{utils::check_text_channel, messages::send_dj_message};

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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
        let queue = handler.queue();
        let _ = queue.stop();
        let _ = handler.stop();

        send_dj_message(&ctx, msg.channel_id, "Queue geleert und Songs gestoppt.".to_string()).await;
        
        if let Err(e) = manager.remove(guild_id).await {
            send_dj_message(&ctx, msg.channel_id, format!("Fehler: {:?}", e).to_string()).await;
        }
        send_dj_message(&ctx, msg.channel_id, "Gut dann geh ich halt.".to_string()).await;
    } else {
        send_dj_message(&ctx, msg.channel_id, "Es gibt nichts zu stoppen.".to_string()).await;
    }

    Ok(())
}