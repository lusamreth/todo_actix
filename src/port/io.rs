use crate::domain::todolist::task::Task;
use crate::domain::todolist::task_date::Taskdate;
use std::fmt;
pub struct DatabaseDoc<T>(T);

impl<T> DatabaseDoc<T> {
    pub fn get_doc(self) -> T {
        return self.0;
    }
    pub fn create(itm: T) -> Self {
        DatabaseDoc(itm)
    }
}
impl<T: fmt::Display> fmt::Debug for DatabaseDoc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Doc-wrapper {}", self.0)
    }
}
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Pipelines<T> {
    pub(crate) pipes: Vec<T>,
}

impl<T> Pipelines<T> {
    pub fn push_pipe(mut self, pipe: T) -> Self {
        self.pipes.push(pipe);
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedOutput {
    pub created_at: Taskdate,
    pub task_id:String,
    // likely to change
    pub modifed_at: Option<Taskdate>,
    pub done: bool,
    //dynamic
    pub progress: f32,
    pub list_name: String,
    pub due_date: Option<Taskdate>,
    pub dued: bool,
    // dynamic!
    pub task_store: Vec<Task>,
}
