use axum_extra::extract::CookieJar;
use domain::task::Task;

use crate::usecase::domain_service::get_user;

pub async fn exec(cookies: CookieJar, body: generated::models::TaskCommand) -> Result<generated::TaskPostResponse, String> {
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
    let task = match body.task_id {
        Some(task_id) => {
            let result = infra::mysql_task_repository::select_one(&task_id).await;
            let task = match result {
                Ok(old_task) => Task::new(
                    Some(old_task.get_task_id().to_string()),
                    body.title,
                    body.description,
                    old_task.get_created_at(),
                    None,
                    old_task.get_created_by().to_string(),
                ),
                Err(_) => {
                    return Ok(generated::TaskPostResponse::Status500(generated::models::Common500Response {
                        message: "Internal Server Error".to_string(),
                    }));
                }
            };
            task
        }
        None => {
            let task = domain::task::Task::new(None, body.title, body.description, None, None, user.get_user_id().to_string());
            task
        }
    };
    if task.is_err() {
        return Ok(generated::TaskPostResponse::Status400(generated::models::Common400Response {
            message: "Bad Request".to_string(),
        }));
    }
    let task = task.unwrap();
    if task.get_created_by() != user.get_user_id() {
        return Ok(generated::TaskPostResponse::Status403(generated::models::Common403Response {
            message: "Unauthorized".to_string(),
        }));
    }
    if infra::mysql_task_repository::upsert_one(&task).await.is_err() {
        return Ok(generated::TaskPostResponse::Status500(generated::models::Common500Response {
            message: "Internal Server Error".to_string(),
        }));
    }

    Ok(generated::TaskPostResponse::Status200(generated::models::Common200Response {
        message: "ok".to_string(),
    }))
}
