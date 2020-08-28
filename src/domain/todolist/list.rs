// the function is : create,append[updater],remove,done,progress,dued,list-tasks
use crate::domain::todolist;
use serde::{Deserialize, Serialize};
use todolist::task_date::Taskdate;
#[derive(Serialize, Deserialize)]
pub struct Todolist {
    pub created_at: Taskdate,
    // likely to change
    pub modifed_at: Taskdate,
    pub done: bool,
    //dynamic
    pub progress: f32,
    pub list_name: String,
    pub due_date: Option<Taskdate>,
    pub dued: bool,
    // dynamic!
    pub task_store: TaskStorage,
}

type Taskid = String;
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStorage {
    pub tasks: Vec<Taskid>,
    pub count: usize,
}

// fields that are avialable to update:
/* modifed_at */
impl TaskStorage {
    fn new() -> Self {
        return TaskStorage {
            tasks: Vec::new(),
            count: 0,
        };
    }
}

#[allow(dead_code)]
impl Todolist {
    pub fn new(name: &str) -> Self {
        return Todolist {
            created_at: Taskdate::new_local(),
            modifed_at: Taskdate::new_local(),
            done: false,
            due_date: None,
            dued: false,
            task_store: TaskStorage::new(),
            list_name: String::from(name),
            progress: 0.00,
        };
    }
    pub fn due(&mut self) {
        self.dued = true
    }
    pub fn finish(&mut self) {
        self.done = true
    }
}

use std::fmt::Debug;
impl Debug for Todolist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"name : {} \n array of task : {:#?} \n progression : {} \n due_state : {} \n when due? : {:#?}",self.list_name,self.task_store.tasks,self.progress,self.dued,self.due_date)
    }
}
