use chrono::NaiveDateTime;
use rand::seq::IteratorRandom;

use crate::error::CustomError;

struct Task {
    pub task_id: String,
    pub title: String,
    pub description: String,
    pub created_by: String,
}
pub struct TaskQuery {
    task: Task,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
pub struct TaskCommand {
    task: Task,
    is_new: bool,
}

const BASE_STR: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

impl TaskCommand {
    pub fn new(task_id: Option<String>, title: String, description: String, created_by: String) -> Result<TaskCommand, CustomError> {
        let (task_id, is_new) = match task_id {
            Some(task_id) => (task_id, false),
            None => {
                let mut rng = &mut rand::thread_rng();
                let id = String::from_utf8(BASE_STR.as_bytes().into_iter().choose_multiple(&mut rng, 8).into_iter().cloned().collect());
                (id.unwrap(), true)
            }
        };

        if title.is_empty() {
            return Err(CustomError::DomainModelError);
        }
        if title.len() > 255 {
            return Err(CustomError::DomainModelError);
        }
        if description.len() > 1000 {
            return Err(CustomError::DomainModelError);
        }
        Ok(TaskCommand {
            task: Task {
                task_id: task_id,
                title: title,
                description: description,
                created_by: created_by,
            },
            is_new: is_new,
        })
    }
    pub fn get_task_id(&self) -> &str {
        &self.task.task_id
    }
    pub fn get_title(&self) -> &str {
        &self.task.title
    }
    pub fn get_description(&self) -> &str {
        &self.task.description
    }
    pub fn get_created_by(&self) -> &str {
        &self.task.created_by
    }
    pub fn is_new(&self) -> bool {
        self.is_new
    }
}

impl TaskQuery {
    pub fn new(task_id: String, title: String, description: String, created_by: String, created_at: NaiveDateTime, updated_at: NaiveDateTime) -> TaskQuery {
        TaskQuery {
            task: Task {
                task_id: task_id,
                title: title,
                description: description,
                created_by: created_by,
            },
            created_at: created_at,
            updated_at: updated_at,
        }
    }
    pub fn get_task_id(&self) -> &str {
        &self.task.task_id
    }
    pub fn get_title(&self) -> &str {
        &self.task.title
    }
    pub fn get_description(&self) -> &str {
        &self.task.description
    }
    pub fn get_created_by(&self) -> &str {
        &self.task.created_by
    }
    pub fn get_created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    pub fn get_updated_at(&self) -> NaiveDateTime {
        self.updated_at
    }
}
