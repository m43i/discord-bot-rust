use serenity::builder::CreateApplicationCommand;
use crate::prisma::{drinks, PrismaClient};

/**
 * Register the add drink command
 */
pub fn register_add_drink(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("trinken")
        .description("Lass dich dran erinnern, dass du trinken sollst.")
}

/**
 * Run the add drink command
 */
pub async fn run_add_drink(user_id: u64, client: &PrismaClient) -> String {
    let drink_user = client.drinks().create(user_id as i64, vec![]).exec().await;

    if drink_user.is_err() {
        return run_remove_drink(user_id, client).await;
    }

    return String::from("Du wurdest zu der Trinkliste hinzugefügt.");
}

/**
 * Run the remove drink command
 */
pub async fn run_remove_drink(user_id: u64, client: &PrismaClient) -> String {
    let delete_user = client.drinks().delete(drinks::id::equals(user_id as i64)).exec().await;
    
    if delete_user.is_err() {
        println!("Error: {:?}", delete_user);
        println!("User ID: {}", user_id as i64);
        return String::from("Du bist nicht auf der Trinkliste.");
    }

    return String::from("Du wurdest von der Trinkliste entfernt.");
}

/**
 * Register the remove drink command
 */
pub fn register_remove_drink(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("verdursten")
        .description("Lass dich nicht mehr dran erinnern, dass du trinken sollst.")
}
