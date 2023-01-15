use anyhow::Result;
use dotenvy::dotenv;
use handler::bot::Bot;
use serenity::{prelude::GatewayIntents, Client};
use std::env;

mod commands;
mod handler;
mod utils;
mod db;

async fn setup_client() -> Client {
    let token = env::var("BOT_TOKEN").expect("Token not provided");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.db")
                .create_if_missing(true),
        )
        .await
        .expect("Couldn't connect to database");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Couldn't run database migrations");

    let bot = Bot { db };
    let client = Client::builder(token, intents)
        .event_handler(bot)
        .await
        .expect("Error creating client");

    return client;
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let mut client = setup_client().await;
    client.start_autosharded().await.expect("Cant start client");

    return Ok(());
}
