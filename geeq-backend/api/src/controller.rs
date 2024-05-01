use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use generated::{models::OauthPostOkResponse, AuthOauthPostResponse};
use infra::{github_adapter::{get_user, post_login_oauth_access_token}, redis_repository};
use uuid::Uuid;

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
    async fn auth_oauth_post(
    &self,
    _method: Method,
    _host: Host,
    _cookies: CookieJar,
    request_body: generated::models::OauthPostRequestBody,
    ) -> Result<generated::AuthOauthPostResponse, String> {
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
        let session_id = generate_geeq_secure_session_id();
        redis_repository::set_session(&session_id, user.unwrap().login);

        //TODO expireを設定する
        let cookie = Cookie::build(("geeq-session-id", session_id))
        .domain("localhost")
        .path("/")
        // .secure(true) // TODO ローカルと本番で使い分けれるようにする
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();

        return Ok(
            AuthOauthPostResponse::Status200{ 
                body: OauthPostOkResponse{
                    message: "ok".to_string(),
                },
                set_cookie: Some(cookie.to_string()),
            }
        )
    }    
}

fn generate_geeq_secure_session_id() -> String {
    let geeq_session_id = Uuid::new_v4().to_string();
    geeq_session_id
}
