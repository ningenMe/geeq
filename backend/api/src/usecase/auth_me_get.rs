use axum_extra::extract::CookieJar;
use domain::auth::Session;
use generated::models::AuthMeGet200Response;
use generated::models::Common401Response;
use generated::models::Common500Response;

use generated::models::User;
use infra::mysql_user_repository;
use infra::redis_repository;

pub async fn exec(cookies: CookieJar) -> Result<generated::AuthMeGetResponse, String> {
    let session = match Session::new_with_session_id(cookies) {
        Ok(session) => session,
        Err(_) => {
            return Ok(generated::AuthMeGetResponse::Status401(Common401Response {
                message: "Unauthenticated".to_string(),
            }));
        }
    };

    let user_id = redis_repository::get_session(&session);
    if user_id.is_none() {
        return Ok(generated::AuthMeGetResponse::Status401(Common401Response {
            message: "Unauthenticated".to_string(),
        }));
    }
    let user = match mysql_user_repository::select(user_id.unwrap()).await {
        Ok(user) => user,
        Err(_) => {
            return Ok(generated::AuthMeGetResponse::Status500(Common500Response {
                message: "Internal Server Error".to_string(),
            }));
        }
    };

    return Ok(generated::AuthMeGetResponse::Status200(AuthMeGet200Response {
        user: User {
            user_id: user.get_user_id().to_string(),
            avatar_url: user.get_avatar_url().to_string(),
        },
    }));
}
