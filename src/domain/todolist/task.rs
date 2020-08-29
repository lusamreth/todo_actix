// mod task_date;
use super::{task_date::Taskdate, BussRes};
use crate::domain::source;
use serde::{Deserialize, Serialize};
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub name: String,
    pub created_at: Taskdate,
    pub modified_at: Taskdate,
    pub(crate) description: String,
    pub completed_at: Option<Taskdate>,
    pub completion: bool,
}

// name could be null or
#[allow(dead_code)]
impl Task {
    pub fn new(name: String, desc: String) -> BussRes<Self, String> {
        let now = Taskdate::new_local();
        let new_id = source::create_id(format!("{}?#{}",&name,&desc));

        if desc.len() < 1 {}
        let created_task = Task {
            task_id: new_id,
            name,
            created_at: now.clone(),
            description: desc,
            completion: false,
            completed_at: None,
            modified_at: now,
        };

        return Ok(created_task);
    }
    pub fn insert_id(&mut self, id: &str) {
        self.task_id.push_str(id);
    }
    pub fn insert_created_at(&mut self, cta: Taskdate) {
        self.created_at = cta;
    }
    pub fn insert_modified_at(&mut self, md_time: Taskdate) {
        self.modified_at = md_time
    }
    pub fn finish(&mut self, completion_time: Taskdate) {
        self.completion = true;
        self.completed_at = Some(completion_time);
    }
    pub fn unfinish(&mut self) {
        self.completion = false;
        // let now = Taskdate::init_date();
        self.completed_at = None;
    }
}
