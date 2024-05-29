use domain::user::User;
use once_cell::sync::Lazy;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::time::Duration;

static NINGENME_MYSQL_MASTER_USER_USERNAME: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_GEEQ_USER_USERNAME").expect("env variable is not found"));
static NINGENME_MYSQL_MASTER_USER_PASSWORD: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_GEEQ_USER_PASSWORD").expect("env variable is not found"));
static NINGENME_MYSQL_HOST: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_HOST").expect("env variable is not found"));
static NINGENME_MYSQL_PORT: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_PORT").expect("env variable is not found"));
static DATABASE_URL: Lazy<String> = Lazy::new(|| {
    format!(
        "mysql://{}:{}@{}:{}/geeq",
        *NINGENME_MYSQL_MASTER_USER_USERNAME, *NINGENME_MYSQL_MASTER_USER_PASSWORD, *NINGENME_MYSQL_HOST, *NINGENME_MYSQL_PORT
    )
});

static POOL: Lazy<Pool<MySql>> = Lazy::new(|| {
    return futures::executor::block_on(async {
        MySqlPoolOptions::new()
            .min_connections(1)
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(5))
            .connect(&*DATABASE_URL)
            .await
            .expect("database is not connected")
    });
});

#[derive(Debug)]
struct UserDto {
    pub user_id: String,
    pub avatar_url: String,
}

pub async fn insert(user: User) -> Result<(), sqlx::Error> {
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
