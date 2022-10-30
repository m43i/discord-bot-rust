use serenity::{framework::standard::{macros::command, CommandResult}, prelude::{Context, Mentionable}, model::prelude::Message};
use songbird::{Event, TrackEvent};
use crate::{lib::{utils::check_text_channel, messages::send_dj_message}, events::track_end::TrackEndNotifier};

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    if !check_text_channel(msg.channel_id) {
        return Ok(());
    }
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            send_dj_message(&ctx, msg.channel_id, "Freundchen du bist in keinem Voice Channel.".to_string()).await;

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        send_dj_message(&ctx, msg.channel_id, format!("Ich bin jetzt in {}.", connect_to.mention()).to_string()).await;

        let chan_id = msg.channel_id;

        let send_http = ctx.http.clone();

        let mut handle = handle_lock.lock().await;

        handle.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier {
                chan_id,
                http: send_http,
            },
        );
    } else {
        send_dj_message(&ctx, msg.channel_id, "Ich habe Probleme beim beitreten.".to_string()).await;
    }

    Ok(())
}