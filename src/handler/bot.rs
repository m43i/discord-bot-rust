use chrono::Utc;
use cron::Schedule;
use serenity::{
    model::prelude::{
        command::Command,
        interaction::{Interaction, InteractionResponseType},
        GuildId, Ready,
    },
    prelude::{Context, EventHandler},
};
use std::str::FromStr;

pub struct Bot {
    pub db: sqlx::SqlitePool,
}

#[serenity::async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let user_id = command.user.id.0;

            let content = match command.data.name.as_str() {
                "trinken" => crate::commands::drink::run_add_drink(user_id.clone(), &self.db).await,
                "verdursten" => {
                    crate::commands::drink::run_remove_drink(user_id.clone(), &self.db).await
                }
                "traum" => {
                    crate::commands::dream::run_add_dream(
                        user_id.clone(),
                        &self.db,
                        &command.data.options,
                    )
                    .await
                }
                "wach" => crate::commands::dream::run_remove_dream(user_id.clone(), &self.db).await,
                _ => String::from("Dazu sage ich mal nichts..."),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            message.content(content).ephemeral(true)
                        })
                })
                .await
            {
                println!("Could not respond to command: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _ = Command::create_global_application_command(&ctx.http.as_ref(), |cmd| {
            crate::commands::drink::register_add_drink(cmd)
        })
        .await;
        let _ = Command::create_global_application_command(&ctx.http.as_ref(), |cmd| {
            crate::commands::drink::register_remove_drink(cmd)
        })
        .await;
        let _ = Command::create_global_application_command(&ctx.http.as_ref(), |cmd| {
            crate::commands::dream::register_remove_dream(cmd)
        })
        .await;
        let _ = Command::create_global_application_command(&ctx.http.as_ref(), |cmd| {
            crate::commands::dream::register_add_dream(cmd)
        })
        .await;
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        println!("Cache is ready!");

        let drink_schedule = Schedule::from_str("0 0 * * * * *").unwrap();
        let dream_schedule = Schedule::from_str("0 30 * * * * *").unwrap();

        let mut next_drink = drink_schedule.upcoming(Utc).next().unwrap();
        let mut next_dream = dream_schedule.upcoming(Utc).next().unwrap();

        let pool_clone = self.db.clone();

        let messages = crate::utils::messages::get_drink_messages();

        tokio::spawn(async move {
            loop {
                if crate::handler::reminder::drink_reminder(
                    &ctx,
                    &pool_clone,
                    &messages,
                    next_drink,
                )
                .await
                {
                    next_drink = drink_schedule.upcoming(Utc).next().unwrap();
                    println!("Next drink reminder: {}", next_drink);
                }

                if crate::handler::reminder::dream_reminder(&ctx, &pool_clone, next_dream).await {
                    next_dream = dream_schedule.upcoming(Utc).next().unwrap();
                    println!("Next dream reminder: {}", next_dream);
                }

                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        });
    }
}
