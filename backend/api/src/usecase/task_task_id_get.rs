use chrono::TimeZone;
use generated::models::{Common400Response, TaskTaskIdGet200Response};
use infra::mysql_task_repository;

pub async fn exec(path_params: generated::models::TaskTaskIdGetPathParams) -> Result<generated::TaskTaskIdGetResponse, String> {
    let task_id = path_params.task_id;

    let task = match mysql_task_repository::select_one(&task_id).await {
        Ok(task) => task,
        Err(_) => {
            return Ok(generated::TaskTaskIdGetResponse::Status400(Common400Response {
                message: "Bad Request".to_string(),
            }));
        }
    };

    return Ok(generated::TaskTaskIdGetResponse::Status200(TaskTaskIdGet200Response {
        task: generated::models::TaskQuery {
            task_id: task.get_task_id().to_string(),
            title: task.get_title().to_string(),
            description: task.get_description().to_string(),
            created_at: chrono::offset::Utc.from_utc_datetime(&task.get_created_at()),
            updated_at: chrono::offset::Utc.from_utc_datetime(&task.get_updated_at()),
            created_by: task.get_created_by().to_string(),
        },
    }));
}
