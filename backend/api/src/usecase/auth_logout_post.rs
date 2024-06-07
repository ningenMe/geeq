use axum_extra::extract::CookieJar;
use domain::auth::Session;
use generated::models::Common200Response;
use generated::models::Common401Response;
use infra::redis_repository;

pub async fn exec(cookies: CookieJar) -> Result<generated::AuthLogoutPostResponse, String> {
    let session = match Session::new_with_session_id(cookies) {
        Ok(session) => session,
        Err(_) => {
            return Ok(generated::AuthLogoutPostResponse::Status401(Common401Response {
                message: "Unauthenticated".to_string(),
            }));
        }
    };

    redis_repository::delete_session(&session);
    return Ok(generated::AuthLogoutPostResponse::Status200(Common200Response { message: "ok".to_string() }));
}
