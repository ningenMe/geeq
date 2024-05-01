use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::{cookie::{Cookie, SameSite}, CookieJar};
use generated::{models::OauthPostOkResponse, AuthOauthPostResponse};
use infra::get_github_access_token;
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
        let res = get_github_access_token(request_body.code).await;
        //TODO access_tokenを使ってuser情報を取得、その後access_tokenは破棄
        //TODO session tokenをredisに永続化
        //TODO expireを設定する
        let session_id = generate_geeq_secure_session_id();
        let cookie = Cookie::build(("geeq-session-id",session_id))
        .domain("localhost")
        .path("/")
        // .secure(true) // TODO ローカルと本番で使い分けれるようにする
        .http_only(true)
        .same_site(SameSite::Strict)
        .build();

        return match res {
            Ok(_) => Ok(
                AuthOauthPostResponse::Status200{ 
                    body: OauthPostOkResponse{
                        message: "ok".to_string(),
                    },
                    set_cookie: Some(cookie.to_string()),
                }
            ),
            Err(_) => {
                todo!()
            }
        }
    }    
}

fn generate_geeq_secure_session_id() -> String {
    let geeq_session_id = Uuid::new_v4().to_string();
    geeq_session_id
}
