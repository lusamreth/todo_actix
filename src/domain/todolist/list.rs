// the function is : create,append[updater],remove,done,progress,dued,list-tasks
use crate::domain::todolist;
use serde::{Deserialize, Serialize};
use todolist::task::Task;
use todolist::task_date::Taskdate;
#[derive(Serialize, Deserialize)]
pub struct Todolist {
    created_at: Taskdate,
    // likely to change
    modifed_at: Option<Taskdate>,
    done: bool,
    //dynamic
    progress: f32,
    list_name: String,
    due_date: Option<Taskdate>,
    dued: bool,
    // dynamic!
    task_store: TaskStorage,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskStorage {
    pub tasks: Vec<Task>,
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
        let new_tk = Taskdate::new_local();
        return Todolist {
            created_at: new_tk,
            modifed_at: None,
            done: false,
            due_date: None,
            dued: false,
            task_store: TaskStorage::new(),
            list_name: String::from(name),
            progress: 0.00,
        };
    }

    pub fn calculate_progress(&self) -> f32 {
        let full_len = self.task_store.count;
        let mut count = 0;
        self.task_store.tasks.iter().for_each(|e| {
            if e.completion == true {
                count += 1;
            }
        });
        return (count / full_len) as f32;
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
