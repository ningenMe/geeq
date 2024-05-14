use axum_extra::extract::cookie::{Cookie, SameSite};
use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::environment::{Environment, ENV};

pub const GEEQ_SESSION_COOKIE_NAME: &str = "geeq-session-id";
pub const GEEQ_SESSION_ID_PREFIX: &str = "geeq:session:";

pub fn generate_geeq_session_id() -> String {
    let geeq_session_id = Uuid::new_v4().to_string();
    geeq_session_id
}

static FRONT_COOKIE_DOMAIN: Lazy<String> = Lazy::new(|| match *ENV {
    Environment::Prod => "ningenme.net".to_string(),
    Environment::Local => "localhost".to_string(),
});

static IS_SECURE: Lazy<bool> = Lazy::new(|| match *ENV {
    Environment::Prod => true,
    Environment::Local => false,
});

static SAME_SITE: Lazy<SameSite> = Lazy::new(|| {
    match *ENV {
        Environment::Prod => SameSite::None, // client/serverで異なるドメインなのでNoneにする
        Environment::Local => SameSite::Strict,
    }
});

pub fn get_geeq_session_cookie<'a>(session_id: &'a str) -> Cookie<'a> {
    let cookie = Cookie::build((GEEQ_SESSION_COOKIE_NAME, session_id))
        .domain(&*FRONT_COOKIE_DOMAIN)
        .path("/")
        .secure(*IS_SECURE)
        .http_only(true)
        .same_site(*SAME_SITE)
        .build();
    cookie
}
