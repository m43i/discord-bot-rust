use serenity::builder::CreateApplicationCommand;

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
pub async fn run_add_drink(user_id: u64, pool: &sqlx::SqlitePool) -> String {
    if crate::db::drink::insert_drink_user(pool, user_id as i64, None).await {
        println!("Added user {} to the drink list", user_id);
        return String::from("Du wurdest zu der Trinkliste hinzugefügt.");
    } else {
        println!("Failed to add user {} to the drink list", user_id);
        return String::from("Du bist bereits auf der Trinkliste.");
    }
}

/**
 * Run the remove drink command
 */
pub async fn run_remove_drink(user_id: u64, pool: &sqlx::SqlitePool) -> String {
    if crate::db::drink::remove_drink_user(pool, user_id as i64).await {
        println!("Removed user {} from the drink list", user_id);
        return String::from("Du wurdest von der Trinkliste entfernt.");
    } else {
        println!("Failed to remove user {} from the drink list", user_id);
        return String::from("Du bist nicht auf der Trinkliste.");
    }
}

/**
 * Register the remove drink command
 */
pub fn register_remove_drink(cmd: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    cmd.name("verdursten")
        .description("Lass dich nicht mehr dran erinnern, dass du trinken sollst.")
}
