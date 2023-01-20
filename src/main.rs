mod commands;
mod handler;
mod prisma;
mod utils;

use anyhow::Result;
use dotenvy::dotenv;
use handler::bot::Bot;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use serenity::{prelude::GatewayIntents, Client};
use std::env;

async fn setup_client() -> Client {
    let token = env::var("BOT_TOKEN").expect("Token not provided");
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_VOICE_STATES;

    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;

    if client.is_err() {
        println!("Error creating prisma client");
        std::process::exit(1);
    }

    let client = client.unwrap();

    let bot = Bot { db: client };
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
