use axum_extra::extract::CookieJar;
use generated::models::AuthMeGet200Response;
use generated::models::Common401Response;
use generated::models::Common500Response;

use generated::models::User;

use super::domain_service::get_user;

pub async fn exec(cookies: CookieJar) -> Result<generated::AuthMeGetResponse, String> {
    let user = match get_user(cookies).await {
        Ok(user) => user,
        Err(err) => match err {
            domain::error::CustomError::Unauthenticated => {
                return Ok(generated::AuthMeGetResponse::Status401(Common401Response {
                    message: "Unauthenticated".to_string(),
                }));
            }
            domain::error::CustomError::InternalServerError => {
                return Ok(generated::AuthMeGetResponse::Status500(Common500Response {
                    message: "Internal Server Error".to_string(),
                }));
            }
            domain::error::CustomError::Unauthorized => todo!(),
            domain::error::CustomError::DomainModelError => todo!(),
        },
    };

    return Ok(generated::AuthMeGetResponse::Status200(AuthMeGet200Response {
        user: User {
            user_id: user.get_user_id().to_string(),
            avatar_url: user.get_avatar_url().to_string(),
        },
    }));
}
