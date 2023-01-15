use serenity::model::{prelude::Member, user::User};
use sqlx::Error;

pub struct DreamUser {
    pub id: i64,
    pub duration: Option<i64>,
    pub message: Option<String>
}

impl PartialEq for DreamUser {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq<Member> for DreamUser {
    fn eq(&self, other: &Member) -> bool {
        self.id == other.user.id.0 as i64
    }
}

impl PartialEq<User> for DreamUser {
    fn eq(&self, other: &User) -> bool {
        self.id == other.id.0 as i64
    }
}

/**
 * Get drem users from the database
 */
pub async fn get_dream_users(pool: &sqlx::sqlite::SqlitePool) -> Vec<DreamUser> {
    let drinks: Result<Vec<DreamUser>, Error> =
        sqlx::query_as!(DreamUser, "SELECT id, duration, message FROM dreams")
            .fetch_all(pool)
            .await;

    if let Ok(drinks) = drinks {
        return drinks;
    } else {
        return vec![];
    }
}

/**
 * Insert a dream user into the database
 */
pub async fn insert_dream_user(
    pool: &sqlx::sqlite::SqlitePool,
    id: i64,
    duration: Option<i64>,
    message: String,
) -> bool {
    let duration = duration.unwrap_or(60);
    let insert = sqlx::query!(
        "INSERT INTO dreams (id, duration, message) VALUES (?, ?, ?)",
        id,
        duration,
        message
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
 * Delete a dream user from the database
 */
pub async fn remove_dream_user(pool: &sqlx::sqlite::SqlitePool, id: i64) -> bool {
    let remove = sqlx::query!("DELETE FROM dreams WHERE id = ?", id)
        .execute(pool)
        .await;

    if let Ok(_) = remove {
        return true;
    } else {
        return false;
    }
}
