use std::sync::Arc;

use serenity::{model::prelude::ChannelId, http::Http};
use songbird::{EventContext, Event, EventHandler as VoiceEventHandler,};

use crate::lib::messages::{send_dj_message_event};

pub struct SongEndNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
}

#[serenity::async_trait]
impl VoiceEventHandler for SongEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        send_dj_message_event(&self.http, self.chan_id, "Der Song ist vorbei.".to_string()).await;

        None
    }
}