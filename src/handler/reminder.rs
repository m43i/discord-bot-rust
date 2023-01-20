use rand::seq::SliceRandom;
use serenity::{model::user::User, prelude::Context, utils::MessageBuilder};

use crate::prisma::PrismaClient;

/**
 * Send drink reminder to a users
 */
pub async fn drink_reminder(ctx: &Context, client: &PrismaClient, messages: &Vec<&str>) {
    let drink_users = client.drinks().find_many(vec![]).exec().await.unwrap_or(vec![]);
    let mut users: Vec<User> = vec![];

    let guilds = ctx.cache.guilds();
    for guild in guilds {
        let guild_id = guild.0;
        let members = crate::utils::channels::get_guild_voice_members(ctx, guild_id).await;

        for member in members {
            for drink_user in &drink_users {
                if drink_user.id != member.user.id.0 as i64 {
                    continue;
                }

                if member.user.bot {
                    continue;
                }

                if users.contains(&member.user) {
                    continue;
                }

                users.push(member.clone().user);
            }
        }
    }

    for user in users {
        println!("Sending drink reminder to {}", user.name);
        let rand_msg = messages.choose(&mut rand::thread_rng()).unwrap();
        let msg = MessageBuilder::new().push(rand_msg).build();
        crate::utils::messages::send_direct_message(ctx, &user, &msg).await;
    }
}

/**
 * Send dream reminder to users
 */
pub async fn dream_reminder(ctx: &Context, client: &PrismaClient) {
    let dream_users = client.dreams().find_many(vec![]).exec().await.unwrap_or(vec![]);
    let mut users: Vec<User> = vec![];

    let guilds = ctx.cache.guilds();
    for guild in guilds {
        let guild_id = guild.0;
        let members = crate::utils::channels::get_guild_voice_members(ctx, guild_id).await;

        for member in members {
            for dream_user in &dream_users {
                if dream_user.id != member.user.id.0 as i64 {
                    continue;
                }

                if member.user.bot {
                    continue;
                }

                if users.contains(&member.user) {
                    continue;
                }

                users.push(member.clone().user);
            }
        }
    }

    for user in users {
        let dream_user = dream_users.iter().find(|&x| x.id == user.id.0 as i64);

        if dream_user.is_none() {
            continue;
        }

        let dream_user = dream_user.unwrap();
        let user_msg = &dream_user
            .message
            .clone()
            .unwrap_or("Denk an deine Traumroutine.".to_string());
        let msg = MessageBuilder::new().push(user_msg).build();
        crate::utils::messages::send_direct_message(ctx, &user, &msg).await;
    }
}
