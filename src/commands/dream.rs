use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::CommandOptionType,
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
    },
};

use crate::prisma::{dreams, PrismaClient};

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
    client: &PrismaClient,
    option: &[CommandDataOption],
) -> String {
    let mut msg: String = "Denk an deine Traumroutine!".to_string();

    if let Some(opt) = option.get(0).and_then(|opt| opt.resolved.as_ref()) {
        if let CommandDataOptionValue::String(s) = &opt {
            msg = s.to_string();
        }
    }

    let dream_user = client
        .dreams()
        .create(
            user_id as i64,
            vec![dreams::message::set(Some(msg.clone()))],
        )
        .exec()
        .await;

    if dream_user.is_err() {
        return run_remove_dream(user_id, client).await;
    }

    return String::from("Du wurdest zu der Traumliste hinzugefügt.");
}

/**
 * Run the remove dream command
 */
pub async fn run_remove_dream(user_id: u64, client: &PrismaClient) -> String {
    let delete_user = client
        .dreams()
        .delete(dreams::id::equals(user_id as i64))
        .exec()
        .await;

    if delete_user.is_err() {
        return String::from("Du bist nicht auf der Traumliste.");
    }

    return String::from("Du wurdest von der Traumliste entfernt.");
}

/**
 * Register the remove dream command
 */
pub fn register_remove_dream(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("wach")
        .description("Lass dich nicht mehr an deine Traumroutine erinnern..")
}
