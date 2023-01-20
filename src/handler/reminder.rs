use chrono::{DateTime, Utc};
use rand::seq::SliceRandom;
use serenity::{model::user::User, prelude::Context, utils::MessageBuilder};

/**
 * Send drink reminder to a users
 */
pub async fn drink_reminder(ctx: &Context, pool: &sqlx::sqlite::SqlitePool, messages: &Vec<&str>) {
    let drink_users = crate::db::drink::get_drink_users(pool).await;
    let mut users: Vec<User> = vec![];

    let guilds = ctx.cache.guilds();
    for guild in guilds {
        let guild_id = guild.0;
        let members = crate::utils::channels::get_voice_members(ctx, guild_id).await;

        for member in members {
            for drink_user in &drink_users {
                if !drink_user.eq(&member) {
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
pub async fn dream_reminder(ctx: &Context, pool: &sqlx::sqlite::SqlitePool) {
    let dream_users = crate::db::dream::get_dream_users(pool).await;
    let mut users: Vec<User> = vec![];

    let guilds = ctx.cache.guilds();
    for guild in guilds {
        let guild_id = guild.0;
        let members = crate::utils::channels::get_voice_members(ctx, guild_id).await;

        for member in members {
            for dream_user in &dream_users {
                if !dream_user.eq(&member) {
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
        let dream_user = dream_users.iter().find(|&x| x.eq(&user));

        if None == dream_user {
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
