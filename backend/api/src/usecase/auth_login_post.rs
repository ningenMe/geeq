use domain::auth::Session;
use generated::models::Common200Response;
use generated::models::Common401Response;
use generated::models::Common500Response;

use generated::AuthLoginPostResponse;
use infra::github_adapter;
use infra::mysql_user_repository;
use infra::redis_repository;

pub async fn exec(request_body: generated::models::AuthLoginPostRequest) -> Result<generated::AuthLoginPostResponse, String> {
    //oauth認証を行う
    let result = github_adapter::post_login_oauth_access_token(request_body.code).await;
    if result.is_err() {
        return Ok(generated::AuthLoginPostResponse::Status401(Common401Response {
            message: "Unauthorized".to_string(),
        }));
    }
    //access_tokenを使ってユーザー情報を取得
    let result = github_adapter::get_user(result.unwrap().access_token).await;
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
