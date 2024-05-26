use domain::{auth::Session, environment::ENV};
use once_cell::sync::Lazy;
use redis::Commands;

static NINGENME_REDIS_HOST: Lazy<String> = Lazy::new(|| match *ENV {
    domain::environment::Environment::Prod => "ningenme-redis".to_string(),
    domain::environment::Environment::Local => "127.0.0.1".to_string(),
});

static REDIS_URL: Lazy<String> = Lazy::new(|| format!("redis://{}:6379/", *NINGENME_REDIS_HOST,));

pub fn set_session(session: &Session, user_id: String) {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();

    let key = get_redis_session_key(session);
    let _: () = connection.set(&key, user_id).unwrap();
    //30日間有効
    let _: () = connection.expire(&key, 3600 * 24 * 30).unwrap();
}

pub fn get_session(session: &Session) -> Option<String> {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();

    let key = get_redis_session_key(session);
    let result: Option<String> = connection.get(&key).unwrap();
    result
}

pub fn delete_session(session: &Session) {
    let client = redis::Client::open(&**REDIS_URL).unwrap();
    let mut connection = client.get_connection().unwrap();

    let key = get_redis_session_key(session);
    let _: () = connection.del(&key).unwrap();
}

fn get_redis_session_key(session: &Session) -> String {
    format!("geeq:session:{}", session.get_session_id())
}
