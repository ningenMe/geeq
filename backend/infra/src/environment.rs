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

pub static POOL: Lazy<Pool<MySql>> = Lazy::new(|| {
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
