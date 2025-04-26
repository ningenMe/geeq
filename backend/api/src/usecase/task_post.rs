use axum_extra::extract::CookieJar;

use crate::usecase::domain_service::get_user;

pub async fn exec(cookies: CookieJar, body: generated::models::TaskCommand) -> Result<generated::TaskPostResponse, String> {
    //userの取得
    let user = match get_user(cookies).await {
        Ok(user) => user,
        Err(err) => match err {
            domain::error::CustomError::Unauthenticated => {
                return Ok(generated::TaskPostResponse::Status401(generated::models::Common401Response {
                    message: "Unauthenticated".to_string(),
                }));
            }
            domain::error::CustomError::InternalServerError => {
                return Ok(generated::TaskPostResponse::Status500(generated::models::Common500Response {
                    message: "Internal Server Error".to_string(),
                }));
            }
            domain::error::CustomError::Unauthorized => todo!(),
            domain::error::CustomError::DomainModelError => todo!(),
        },
    };
    //更新したいtaskの作成
    let update_task = match domain::task::TaskCommand::new(body.task_id, body.title, body.description, body.options, user.get_user_id().to_string()) {
        Ok(task) => task,
        Err(_) => {
            return Ok(generated::TaskPostResponse::Status400(generated::models::Common400Response {
                message: "Bad Request".to_string(),
            }));
        }
    };

    //更新時の処理
    if !update_task.is_new() {
        let old_task = match infra::mysql_task_repository::select_one(update_task.get_task_id()).await {
            Ok(optional_task) => match optional_task {
                Some(task) => task,
                None => {
                    return Ok(generated::TaskPostResponse::Status400(generated::models::Common400Response {
                        message: "Not Found".to_string(),
                    }));
                }
            },
            Err(_) => {
                return Ok(generated::TaskPostResponse::Status500(generated::models::Common500Response {
                    message: "Internal Server Error".to_string(),
                }));
            }
        };

        //更新権限の確認
        if old_task.get_created_by() != user.get_user_id() {
            return Ok(generated::TaskPostResponse::Status403(generated::models::Common403Response {
                message: "Unauthorized".to_string(),
            }));
        }
    }

    //taskの更新
    if infra::mysql_task_repository::upsert_one(&update_task).await.is_err() {
        return Ok(generated::TaskPostResponse::Status500(generated::models::Common500Response {
            message: "Internal Server Error".to_string(),
        }));
    }

    Ok(generated::TaskPostResponse::Status200(generated::models::Common200Response {
        message: "ok".to_string(),
    }))
}
