use axum_extra::extract::cookie::{Cookie, SameSite};
use uuid::Uuid;

pub const GEEQ_SESSION_COOKIE_NAME: &str = "geeq-session-id";
pub const GEEQ_SESSION_ID_PREFIX: &str = "geeq:session:";
pub const GEEQ_SESSION_ID_EXPIRE_SECONDS: u32 = 3600 * 24 * 30;

pub fn generate_geeq_session_id() -> String {
    let geeq_session_id = Uuid::new_v4().to_string();
    geeq_session_id
}

pub fn get_geeq_session_cookie<'a>(session_id: &'a str) -> Cookie<'a> {
    let cookie = Cookie::build((GEEQ_SESSION_COOKIE_NAME, session_id))
        .domain("localhost")
        .path("/")
        // .secure(true) // TODO ローカルと本番で使い分けれるようにする
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();
    cookie       
}