use serenity::{framework::standard::{macros::command, CommandResult, Args}, prelude::Context, model::prelude::Message};
use crate::lib::{utils::check_text_channel, messages::send_dj_message};

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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

        if queue.len() == 0 {
            send_dj_message(&ctx, msg.channel_id, "Das ist der letzte Song, ich kann nicht weiter springen.".to_string()).await;
            let _ = queue.stop();
            let _ = handler.stop();
        } else {
            let _ = queue.skip();
            send_dj_message(&ctx, msg.channel_id, format!("Songs übersrpungen: {} noch in der Warteschlange.", queue.len()).to_string()).await;
        }
    } else {
        send_dj_message(&ctx, msg.channel_id, "Es gibt nichts zu skippen".to_string()).await;
    }

    Ok(())
}