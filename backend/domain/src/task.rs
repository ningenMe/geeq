use chrono::NaiveDateTime;
use rand::seq::IteratorRandom;

use crate::error::CustomError;

pub struct Task {
    task_id: String,
    title: String,
    description: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
    created_by: String,
}
const BASE_STR: &str = "abcdefghijklmnopqrstuvwxyz0123456789";

impl Task {
    pub fn new(
        task_id: Option<String>,
        title: String,
        description: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
        created_by: String,
    ) -> Result<Task, CustomError> {
        let task_id = match task_id {
            Some(task_id) => task_id,
            None => {
                let mut rng = &mut rand::thread_rng();
                let id = String::from_utf8(BASE_STR.as_bytes().into_iter().choose_multiple(&mut rng, 8).into_iter().cloned().collect());
                id.unwrap()
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
        Ok(Task {
            task_id: task_id,
            title: title,
            description: description,
            created_at: created_at,
            updated_at: updated_at,
            created_by: created_by,
        })
    }
    pub fn get_task_id(&self) -> &str {
        &self.task_id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }
    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
}
