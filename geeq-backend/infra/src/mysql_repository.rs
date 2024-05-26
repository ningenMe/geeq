use domain::user::User;
use once_cell::sync::Lazy;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::time::Duration;

static NINGENME_MYSQL_MASTER_USER_USERNAME: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_MASTER_USER_USERNAME").expect("env variable is not found"));
static NINGENME_MYSQL_MASTER_USER_PASSWORD: Lazy<String> = Lazy::new(|| env::var("NINGENME_MYSQL_MASTER_USER_PASSWORD").expect("env variable is not found"));
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

pub async fn insert(user: User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO blog (user_id, avatar_url)
         VALUES (?, ?) AS new ON DUPLICATE KEY UPDATE avatar_url = new.avatar_url ",
        user.login,
        user.avatar_url
    )
    .execute(&*POOL)
    .await?;
    return Ok(());
}

pub async fn select(user_id: String) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(User, "SELECT user_id, avatar_url FROM user WHERE user_id = ? ")
        .fetch_all(&*POOL)
        .await?;
    return Ok(user);
}
