use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::CookieJar;
use generated::{models::OauthPostOkResponse, AuthOauthPostResponse};
use infra::get_github_access_token;

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

        return match res {
            Ok(res) => Ok(AuthOauthPostResponse::Status200(
                OauthPostOkResponse{
                    message: res,
                }
            )),
            Err(res) => {
                todo!()
            }
        }
    }    
}
