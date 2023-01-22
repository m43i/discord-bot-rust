use chrono::Utc;
use prisma_client_rust::NewClientError;
use cron::Schedule;
use crate::prisma::PrismaClient;
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
    pub db: PrismaClient,
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

        let drink_schedule_1 = Schedule::from_str("0 0/45 0/3 * * * *").unwrap();
        let drink_schedule_2 = Schedule::from_str("0 30 1/3 * * * *").unwrap();
        let drink_schedule_3 = Schedule::from_str("0 15 2/3 * * * *").unwrap();
        let dream_schedule = Schedule::from_str("0 30 * * * * *").unwrap();

        let mut next_drinks = vec![
            drink_schedule_1.upcoming(Utc).next().unwrap(),
            drink_schedule_2.upcoming(Utc).next().unwrap(),
            drink_schedule_3.upcoming(Utc).next().unwrap(),
        ];
        let mut next_dream = dream_schedule.upcoming(Utc).next().unwrap();

        let messages = crate::utils::messages::get_drink_messages();

        println!("Schedules: {:?}, {:?}", next_drinks, next_dream);

        tokio::spawn(async move {
            let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

            if client.is_err() {
                println!("Could not connect to database: {:?}", client.err());
                return;
            }

            let client = client.unwrap();

            loop {
                for i in 0..next_drinks.len() {
                    if Utc::now() > next_drinks[i] {
                        crate::handler::reminder::drink_reminder(&ctx, &client, &messages).await;
                        if let Some(next) = drink_schedule_1.upcoming(Utc).next() {
                            next_drinks[i] = next;
                            println!("Next drink: {:?}", next_drinks[i]);
                        }
                    }
                }

                if Utc::now() > next_dream {
                    crate::handler::reminder::dream_reminder(&ctx, &client).await;
                    if let Some(next) = dream_schedule.upcoming(Utc).next() {
                        next_dream = next;
                        println!("Next dream: {:?}", next_dream);
                    }
                }

                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        });
    }
}
