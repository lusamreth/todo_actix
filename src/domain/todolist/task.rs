// mod task_date;
use super::task_date::Taskdate;
use serde::Serialize;
#[allow(dead_code)]
#[derive(Debug,Serialize)]
pub struct Task {
    // task_id:String,
    name: String,
    created_at: Taskdate,
    description: String,
    completed_at: Option<Taskdate>,
    pub completion: bool,
}
// name could be null or
#[allow(dead_code)]
impl Task {
    fn new(name: String, desc: String) -> Self {
        let now = Taskdate::new_local();
        return {
            Task {
                name,
                created_at: now,
                description: desc,
                completion: false,
                completed_at: None,
            }
        };
    }
    fn finish(&mut self) {
        self.completion = true;
        let now = Taskdate::new_local();
        self.completed_at = Some(now);
    }
    fn unfinish(&mut self) {
        self.completion = false;
        // let now = Taskdate::init_date();
        self.completed_at = None;
    }
}
