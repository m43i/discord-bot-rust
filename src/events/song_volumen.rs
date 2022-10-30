use std::sync::Arc;

use serenity::{model::prelude::ChannelId, http::Http};
use songbird::{EventContext, Event, EventHandler as VoiceEventHandler,};

use crate::lib::messages::{send_dj_message_event};

pub struct SongVolume {
    chan_id: ChannelId,
    http: Arc<Http>,
    volumen: f32,
}

#[serenity::async_trait]
impl VoiceEventHandler for SongVolume {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(&[(state, track)]) = ctx {
            let _ = track.set_volume(self.volumen);
            if state.volume < 1e-2 {
                let _ = track.stop();
                send_dj_message_event(&self.http, self.chan_id, "Stoppe Song...".to_string()).await;
                Some(Event::Cancel)
            } else {
                send_dj_message_event(&self.http, self.chan_id, "Lautstärke reduziert.".to_string()).await;
                None
            }
        } else {
            None
        }
    }
}