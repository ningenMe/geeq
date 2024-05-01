use once_cell::sync::Lazy;
use redis::Commands;
use std::env;

static NINGENME_REDIS_HOST: Lazy<String> = Lazy::new(|| {
    //TODO 本番/localで書き分ける
    match env::var("NINGENME_REDIS_HOST") {
        Ok(it) => it,
        Err(_) => "127.0.0.1".to_string(),
    }
});

static REDIS_URL: Lazy<String> = Lazy::new(|| {
    format!(
        "redis://{}:6379/",
        *NINGENME_REDIS_HOST,
    )
});

pub fn set_session(session_id: &str, user_id: String) {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();

    let key = format!("geeq:session:{}", session_id);
    let _: () = connection.set(&key, user_id).unwrap();
    //30日間有効
    let _: () = connection.expire(&key, 3600 * 24 * 30).unwrap(); 
}