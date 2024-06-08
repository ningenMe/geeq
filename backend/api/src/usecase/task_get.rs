use chrono::TimeZone;
use generated::models::{Common500Response, TaskGet200Response};
use infra::mysql_task_repository;

pub async fn exec() -> Result<generated::TaskGetResponse, String> {
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
            .map(|task| generated::models::TaskQuery {
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
