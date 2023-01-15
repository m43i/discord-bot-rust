use serenity::{prelude::Context, model::{prelude::ChannelId, user::User}, utils::MessageBuilder};

/**
 * Return a list of messages for the drink reminder
 */
pub fn get_drink_messages() -> Vec<&'static str> {
    return vec![        
        "Ein Gläschen in Ehren kann niemand verwehren.",
        "Von der Mitte zur Titte zum Sack, zack, zack!",
        "Euch ist bekannt, was wir bedürfen, wir wollen starke Getränke schlürfen.",
        "Wer Liebe mag und Einigkeit, der trinkt auch mal ne Kleinigkeit.",
        "Essen ist ein Bedürfnis des Magens, Trinken ein Bedürfnis der Seele. Essen ist ein gewöhnliches Handwerk, Trinken eine Kunst.",
        "Zu viel kann man nie trinken, doch trinkt man nie genug!",
        "Es tut mir im Herz so weh, wenn ich vom Glas den Boden seh.",
        "Hau wech die Scheiße!",
        "Du bist dehydriert? Trink Hydration!",
        "N Sekt vielleicht?",
        "Du siehst schlapp aus trink mal lieber was.",
        "Ey Mädels, trinken nicht vergessen.",
        "El Deniz hat bestimmt was in seinem Bauchladen!"
    ];
}

/**
 * Send a message to a channel
 */
#[allow(dead_code)]
pub async fn send_message(ctx: &Context, channel_id: &ChannelId, message: &MessageBuilder) {
    let msg = channel_id.say(&ctx.http, &message).await;
    
    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
}

/**
 * Send a direct message to a user
 */
pub async fn send_direct_message(ctx: &Context, user: &User, message: &String) {
    let msg = user.direct_message(&ctx.http, |m| {
        m.content(&message);
        m
    }).await;
    
    if let Err(why) = msg {
        println!("Error sending direct message: {:?}", why);
    }
}
