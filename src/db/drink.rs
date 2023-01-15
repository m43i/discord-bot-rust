use serenity::model::prelude::Member;
use sqlx::Error;

pub struct DrinkUser {
    pub id: i64,
    pub duration: Option<i64>,
}

impl PartialEq for DrinkUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<Member> for DrinkUser {
    fn eq(&self, other: &Member) -> bool {
        self.id == other.user.id.0 as i64
    }
}

/**
 * Get drink users from the database
 */
pub async fn get_drink_users(pool: &sqlx::sqlite::SqlitePool) -> Vec<DrinkUser> {
    let drinks: Result<Vec<DrinkUser>, Error> =
        sqlx::query_as!(DrinkUser, "SELECT id, duration FROM drinks")
            .fetch_all(pool)
            .await;

    if let Ok(drinks) = drinks {
        return drinks;
    } else {
        return vec![];
    }
}

/**
 * Insert a drink user into the database
 */
pub async fn insert_drink_user(
    pool: &sqlx::sqlite::SqlitePool,
    id: i64,
    duration: Option<i64>,
) -> bool {
    let duration = duration.unwrap_or(60);
    let insert = sqlx::query!(
        "INSERT INTO drinks (id, duration) VALUES (?, ?)",
        id,
        duration
    )
    .execute(pool)
    .await;

    if let Ok(_insert) = insert {
        return true;
    } else {
        return false;
    }
}

/**
 * Update a drink user in the database
 */
pub async fn remove_drink_user(pool: &sqlx::sqlite::SqlitePool, id: i64) -> bool {
    let remove = sqlx::query!("DELETE FROM drinks WHERE id = ?", id)
        .execute(pool)
        .await;

    if let Ok(_) = remove {
        return true;
    } else {
        return false;
    }
}

