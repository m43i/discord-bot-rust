use std::sync::Arc;

use serenity::{model::prelude::ChannelId, http::Http};
use songbird::{EventContext, Event, EventHandler as VoiceEventHandler,};

use crate::lib::messages::{send_dj_message_event};

pub struct TrackEndNotifier {
    pub chan_id: ChannelId,
    pub http: Arc<Http>,
}

#[serenity::async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            send_dj_message_event(&self.http, self.chan_id, format!("Songs gespielt: {}.", track_list.len()).to_string()).await;
        }

        None
    }
}