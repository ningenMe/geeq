use chrono::NaiveDateTime;
use domain::task::{TaskCommand, TaskQuery};

use crate::environment::POOL;

#[derive(Debug)]
struct TaskDto {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub options: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn select_one(task_id: &str) -> Result<Option<TaskQuery>, sqlx::Error> {
    let dto = sqlx::query_as!(
        TaskDto,
        "SELECT task_id, title, description, options, created_by, created_at, updated_at FROM task WHERE task_id = ?",
        task_id
    )
    .fetch_optional(&*POOL)
    .await?;

    return match dto {
        Some(dto) => Ok(Some(TaskQuery::new(
            dto.task_id,
            dto.title,
            dto.description,
            dto.options,
            dto.created_by,
            dto.created_at,
            dto.updated_at,
        ))),
        None => Ok(None),
    };
}

pub async fn select_all() -> Result<Vec<TaskQuery>, sqlx::Error> {
    let dtos = sqlx::query_as!(
        TaskDto,
        "SELECT task_id, title, description, options, created_by, created_at, updated_at FROM task"
    )
    .fetch_all(&*POOL)
    .await?;
    return Ok(dtos
        .into_iter()
        .map(|dto| {
            TaskQuery::new(
                dto.task_id,
                dto.title,
                dto.description,
                dto.options,
                dto.created_by,
                dto.created_at,
                dto.updated_at,
            )
        })
        .collect());
}

pub async fn upsert_one(task: &TaskCommand) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO task (task_id, title, description, options, created_by) VALUES (?, ?, ?, ?, ?) AS new ON DUPLICATE KEY UPDATE title = new.title, description = new.description, options = new.options, created_by = new.created_by",
        task.get_task_id(),
        task.get_title(),
        task.get_description(),
        task.get_options_string(),
        task.get_created_by(),
    )
    .execute(&*POOL)
    .await?;
    Ok(())
}
