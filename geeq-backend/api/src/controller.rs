use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::CookieJar;
use domain::auth::{generate_geeq_session_id, get_geeq_session_cookie, GEEQ_SESSION_COOKIE_NAME};
use generated::{models::{AuthMeGet200Response, Common200Response, Common401Response}, AuthLoginPostResponse};
use infra::{github_adapter::{get_user, post_login_oauth_access_token}, redis_repository};

#[derive(Clone)]
pub struct Api {}
impl AsRef<Api> for Api {
    #[inline]
    fn as_ref(&self) -> &Api {
        self
    }
}

#[async_trait]
impl generated::Api for Api where 
{   
    async fn auth_login_post(
    &self,
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    request_body: generated::models::AuthLoginPostRequestBody,
    ) -> Result<generated::AuthLoginPostResponse, String> {
        //oauth認証を行う
        let result = post_login_oauth_access_token(request_body.code).await;
        if result.is_err() {
            return Err("error".to_string());
        }
        //access_tokenを使ってユーザー情報を取得
        let user = get_user(result.unwrap().access_token).await;
        if user.is_err() {
            return Err("error".to_string());
        }
        //TODO ユーザー情報をmysqlに保存
        
        //ユーザー情報を使ってsession_idを生成した後、redisに保存
        let session_id = generate_geeq_session_id();
        redis_repository::set_session(&session_id, user.unwrap().login);

        let cookie = get_geeq_session_cookie(&session_id);

        return Ok(
            AuthLoginPostResponse::Status200 { 
                body: Common200Response{
                    message: "ok".to_string(),
                },
                set_cookie: Some(cookie.to_string()),
            }
        )
    }
    
    async fn auth_me_get(
    &self,
    _method: Method,
    _host: Host,
    cookies: CookieJar,
    ) -> Result<generated::AuthMeGetResponse, String> {
        let session_id = cookies.get(GEEQ_SESSION_COOKIE_NAME);
        if session_id.is_none() {
            return Ok(generated::AuthMeGetResponse::Status401(Common401Response
                { message: "Unauthenticated".to_string() }
            ));
        }
        let user_id = redis_repository::get_session(session_id.unwrap().value());
        if user_id.is_none() {
            return Ok(generated::AuthMeGetResponse::Status401(Common401Response
                { message: "Unauthenticated".to_string() }
            ));
        }
        return Ok(generated::AuthMeGetResponse::Status200(AuthMeGet200Response
            { user_id: user_id.unwrap() }
        ));
    }
    
    async fn auth_logout_post(
    &self,
    _method: Method,
    _host: Host,
    cookies: CookieJar,
    ) -> Result<generated::AuthLogoutPostResponse, String> {
        let session_id = cookies.get(GEEQ_SESSION_COOKIE_NAME);
        if session_id.is_none() {
            return Ok(generated::AuthLogoutPostResponse::Status401(Common401Response
                { message: "Unauthenticated".to_string() }
            ));
        }
        redis_repository::delete_session(session_id.unwrap().value());
        return Ok(generated::AuthLogoutPostResponse::Status200(Common200Response
            { message: "ok".to_string() }
        ));
    }    
}
