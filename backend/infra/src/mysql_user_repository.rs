use domain::user::User;

use crate::environment::POOL;

#[derive(Debug)]
struct UserDto {
    pub user_id: String,
    pub avatar_url: String,
}

pub async fn insert(user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO user (user_id, avatar_url)
         VALUES (?, ?) AS new ON DUPLICATE KEY UPDATE avatar_url = new.avatar_url ",
        user.get_user_id(),
        user.get_avatar_url()
    )
    .execute(&*POOL)
    .await?;
    return Ok(());
}

pub async fn select(user_id: String) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(UserDto, "SELECT user_id, avatar_url FROM user WHERE user_id = ? ", user_id)
        .fetch_one(&*POOL)
        .await?;
    return Ok(User::new(user.user_id, user.avatar_url));
}
