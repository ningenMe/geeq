use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::CookieJar;

use crate::usecase;

#[derive(Clone)]
pub struct Api {}
impl AsRef<Api> for Api {
    #[inline]
    fn as_ref(&self) -> &Api {
        self
    }
}

#[async_trait]
impl generated::Api for Api {
    async fn auth_login_post(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        request_body: generated::models::AuthLoginPostRequest,
    ) -> Result<generated::AuthLoginPostResponse, String> {
        return usecase::auth_login_post::exec(request_body).await;
    }

    async fn auth_me_get(&self, _method: Method, _host: Host, cookies: CookieJar) -> Result<generated::AuthMeGetResponse, String> {
        return usecase::auth_me_get::exec(cookies).await;
    }

    async fn auth_logout_post(&self, _method: Method, _host: Host, cookies: CookieJar) -> Result<generated::AuthLogoutPostResponse, String> {
        return usecase::auth_logout_post::exec(cookies).await;
    }

    async fn task_get(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<generated::TaskGetResponse, String> {
        return usecase::task_get::exec().await;
    }

    async fn task_task_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: generated::models::TaskTaskIdGetPathParams,
    ) -> Result<generated::TaskTaskIdGetResponse, String> {
        return usecase::task_task_id_get::exec(path_params).await;
    }

    async fn task_post(
        &self,
        _method: Method,
        _host: Host,
        cookies: CookieJar,
        body: generated::models::TaskCommand,
    ) -> Result<generated::TaskPostResponse, String> {
        return usecase::task_post::exec(cookies, body).await;
    }
}
