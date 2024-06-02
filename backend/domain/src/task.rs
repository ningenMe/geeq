use chrono::NaiveDateTime;

pub struct Task {
    task_id: String,
    title: String,
    description: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    created_by: String,
}
impl Task {
    pub fn new(task_id: String, title: String, description: String, created_at: NaiveDateTime, updated_at: NaiveDateTime, created_by: String) -> Task {
        //TODO バリデーション
        Task {
            task_id: task_id,
            title: title,
            description: description,
            created_at: created_at,
            updated_at: updated_at,
            created_by: created_by,
        }
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
    pub fn get_created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
    pub fn get_updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }
    pub fn get_created_by(&self) -> &str {
        &self.created_by
    }
}
