use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::CookieJar;
use chrono::TimeZone;
use domain::auth::Session;
use generated::models::{AuthMeGet200Response, Common200Response, Common401Response, Common500Response, TaskGet200Response, User};
use infra::{mysql_task_repository, mysql_user_repository, redis_repository};

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
        return usecase::auth_login_post::auth_login_post(request_body).await;
    }

    async fn auth_me_get(&self, _method: Method, _host: Host, cookies: CookieJar) -> Result<generated::AuthMeGetResponse, String> {
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

    async fn auth_logout_post(&self, _method: Method, _host: Host, cookies: CookieJar) -> Result<generated::AuthLogoutPostResponse, String> {
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

    async fn task_get(&self, _method: Method, _host: Host, _cookies: CookieJar) -> Result<generated::TaskGetResponse, String> {
        let tasks = match mysql_task_repository::select_all().await {
            Ok(tasks) => tasks,
            Err(_) => {
                return Ok(generated::TaskGetResponse::Status500(Common500Response {
                    message: "Internal Server Error".to_string(),
                }));
            }
        };
        return Ok(generated::TaskGetResponse::Status200(TaskGet200Response {
            tasks: tasks
                .into_iter()
                .map(|task| generated::models::Task {
                    task_id: task.get_task_id().to_string(),
                    title: task.get_title().to_string(),
                    description: task.get_description().to_string(),
                    created_at: chrono::offset::Utc.from_utc_datetime(&task.get_created_at()),
                    updated_at: chrono::offset::Utc.from_utc_datetime(&task.get_updated_at()),
                    created_by: task.get_created_by().to_string(),
                })
                .collect(),
        }));
    }

    async fn task_task_id_get(
        &self,
        _method: Method,
        _host: Host,
        _cookies: CookieJar,
        path_params: generated::models::TaskTaskIdGetPathParams,
    ) -> Result<generated::TaskTaskIdGetResponse, String> {
        return usecase::task_task_id_get::task_task_id_get(path_params).await;
    }
}
