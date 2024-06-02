use chrono::NaiveDateTime;
use domain::task::Task;

use crate::environment::POOL;

#[derive(Debug)]
struct TaskDto {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub created_by: String,
}

pub async fn select() -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(TaskDto, "SELECT task_id, title, description, created_at, updated_at, created_by FROM task")
        .fetch_all(&*POOL)
        .await?;
    return Ok(tasks
        .into_iter()
        .map(|task| Task::new(task.task_id, task.title, task.description, task.created_at, task.updated_at, task.created_by))
        .collect());
}
