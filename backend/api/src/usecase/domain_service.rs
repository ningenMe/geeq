use axum_extra::extract::CookieJar;
use domain::{auth::Session, error::CustomError, user::User};
use infra::{mysql_user_repository, redis_repository};

pub(crate) async fn get_user(cookies: CookieJar) -> Result<User, CustomError> {
    let session = match Session::new_with_session_id(cookies) {
        Ok(session) => session,
        Err(_) => {
            return Err(CustomError::Unauthenticated);
        }
    };

    let user_id = redis_repository::get_session(&session);
    if user_id.is_none() {
        return Err(CustomError::Unauthenticated);
    }
    let user = match mysql_user_repository::select(user_id.unwrap()).await {
        Ok(user) => user,
        Err(_) => {
            return Err(CustomError::InternalServerError);
        }
    };

    return Ok(user);
}
