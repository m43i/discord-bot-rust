use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType, interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

/**
 * Register the add dream command
 */
pub fn register_add_dream(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("traum")
        .description("Lass dich dran erinnern, deine Traumroutine zu machen.")
        .create_option(|opt| {
            opt.name("nachricht")
                .description("Die Nachricht für deine Traumroutine.")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

/**
 * Run the add dream command
 */
pub async fn run_add_dream(
    user_id: u64,
    pool: &sqlx::SqlitePool,
    option: &[CommandDataOption],
) -> String {
    let mut msg: String = "Denk an deine Traumroutine!".to_string();
    
    if let Some(opt) = option.get(0).and_then(|opt| opt.resolved.as_ref()) {
        if let CommandDataOptionValue::String(s) = &opt {
            msg = s.to_string();
        }
    }

    if crate::db::dream::insert_dream_user(pool, user_id as i64, None, msg).await {
        println!("Added user {} to the dream list", user_id);
        return String::from("Du wurdest zu der Traumliste hinzugefügt.");
    } else {
        println!("Failed to add user {} to the dream list", user_id);
        return String::from("Du bist bereits auf der Traumliste.");
    }
}

/**
 * Run the remove dream command
 */
pub async fn run_remove_dream(user_id: u64, pool: &sqlx::SqlitePool) -> String {
    if crate::db::dream::remove_dream_user(pool, user_id as i64).await {
        println!("Removed user {} from the dream list", user_id);
        return String::from("Du wurdest von der Traumliste entfernt.");
    } else {
        println!("Failed to remove user {} from the dream list", user_id);
        return String::from("Du bist nicht auf der Traumliste.");
    }
}

/**
 * Register the remove dream command
 */
pub fn register_remove_dream(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("wach")
        .description("Lass dich nicht mehr an deine Traumroutine erinnern..")
}
