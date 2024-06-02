use async_trait::async_trait;
use axum::{extract::Host, http::Method};

use axum_extra::extract::CookieJar;
use chrono::TimeZone;
use domain::auth::Session;
use generated::{
    models::{AuthMeGet200Response, Common200Response, Common401Response, Common500Response, TaskGet200Response, User},
    AuthLoginPostResponse,
};
use infra::{
    github_adapter::{get_user, post_login_oauth_access_token},
    mysql_user_repository::{self},
    redis_repository,
};

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
        //oauth認証を行う
        let result = post_login_oauth_access_token(request_body.code).await;
        if result.is_err() {
            return Ok(generated::AuthLoginPostResponse::Status401(Common401Response {
                message: "Unauthorized".to_string(),
            }));
        }
        //access_tokenを使ってユーザー情報を取得
        let result = get_user(result.unwrap().access_token).await;
        if result.is_err() {
            return Ok(generated::AuthLoginPostResponse::Status401(Common401Response {
                message: "Unauthorized".to_string(),
            }));
        }
        let user = result.unwrap();

        if let Err(err) = mysql_user_repository::insert(&user).await {
            return Ok(generated::AuthLoginPostResponse::Status500(Common500Response { message: err.to_string() }));
        };

        //ユーザー情報を使ってsession_idを生成した後、redisに保存
        let session = Session::new();
        redis_repository::set_session(&session, user.get_user_id());

        let cookie = session.get_geeq_session_cookie();

        return Ok(AuthLoginPostResponse::Status200 {
            body: Common200Response { message: "ok".to_string() },
            set_cookie: Some(cookie.to_string()),
        });
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
        let tasks = match infra::mysql_task_repository::select().await {
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
}
