use serenity::{framework::standard::{macros::command, CommandResult}, prelude::Context, model::prelude::Message};
use crate::lib::{utils::check_text_channel, messages::send_dj_message};

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            send_dj_message(&ctx, msg.channel_id, format!("Fehler: {:?}", e).to_string()).await;
        }

        send_dj_message(&ctx, msg.channel_id, "Gut dann geh ich halt.".to_string()).await;
    } else {
        send_dj_message(&ctx, msg.channel_id, "Ich bin in keinem Channel.".to_string()).await;
    }

    Ok(())
}