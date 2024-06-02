use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use once_cell::sync::Lazy;
use uuid::{Error, Uuid};

use crate::environment::{Environment, ENV};

pub const GEEQ_SESSION_COOKIE_NAME: &str = "geeq-session-id";
pub const GEEQ_SESSION_ID_PREFIX: &str = "geeq:session:";

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

#[derive(Debug, Clone)]
pub struct DomainModelError {
    message: String,
}

impl From<Error> for DomainModelError {
    fn from(err: Error) -> Self {
        Self { message: err.to_string() }
    }
}

impl std::error::Error for DomainModelError {}

impl std::fmt::Display for DomainModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct Session {
    session_id_uuid: Uuid,
}

impl Session {
    pub fn new() -> Self {
        let session_id_uuid = Uuid::new_v4();
        Self { session_id_uuid }
    }

    pub fn new_with_session_id(cookies: CookieJar) -> Result<Self, DomainModelError> {
        let session_id = match cookies.get(GEEQ_SESSION_COOKIE_NAME) {
            Some(cookie) => cookie.value(),
            None => {
                return Err(DomainModelError {
                    message: "geeq session cookie is wrong".to_string(),
                });
            }
        };

        let session_id_uuid = Uuid::parse_str(&session_id);
        return match session_id_uuid {
            Ok(uuid) => Ok(Self { session_id_uuid: uuid }),
            Err(err) => Err(DomainModelError::from(err)),
        };
    }

    pub fn get_geeq_session_cookie(&self) -> Cookie<'static> {
        let cookie = Cookie::build((GEEQ_SESSION_COOKIE_NAME, self.session_id_uuid.to_string()))
            .domain(&*FRONT_COOKIE_DOMAIN)
            .path("/")
            .secure(*IS_SECURE)
            .http_only(true)
            .same_site(*SAME_SITE)
            .build();
        cookie
    }

    pub fn get_session_id(&self) -> String {
        return self.session_id_uuid.to_string();
    }
}
