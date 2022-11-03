use serenity::{
    model::prelude::{ChannelId, Member},
    prelude::Context,
    utils::MessageBuilder,
};

pub async fn send_drink_message(
    ctx: &Context,
    channel_id: ChannelId,
    message: String,
    members: Vec<Member>,
) {
    let mut messgae_builder = MessageBuilder::new()
        .push_bold("Trink-Reminder: ")
        .push_line(message)
        .clone();

    for member in members {
        messgae_builder.mention(&member);
        messgae_builder.push(" ");
    }
    let msg = messgae_builder.build();

    channel_id.say(&ctx.http, msg).await.unwrap();
}

pub async fn send_dj_message(ctx: &Context, channel_id: ChannelId, message: String) {
    let msg = MessageBuilder::new()
        .push_bold("DJ: ")
        .push(message)
        .build();
    channel_id.say(&ctx.http, msg).await.unwrap();
}
