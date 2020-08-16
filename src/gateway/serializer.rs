use crate::domain::todolist::{self, list::TaskStorage, task::Task};
use mongodb::bson::{doc, Document};
use serde::Serialize;
use std::fmt;
use todolist::task_date::Taskdate;
#[derive(Serialize)]
struct TaskStorepartial {
    pub task_store: TaskStorage,
}
// abstraction layer built to remove dependencies
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
use mongodb::bson::Bson;
impl From<Task> for Document {
    fn from(tk: Task) -> Self {
        let mdf_str = tk.modified_at.to_string();
        let cmpt = match tk.completed_at {
            Some(tkat) => Bson::String(tkat.to_string()),
            None => Bson::Null,
        };
        let doc = doc! {
            "name":tk.name,
            "complete_at":cmpt,
            "created_at":tk.created_at.to_string(),
            "modified_at":mdf_str,
            "completion" : tk.completion
        };
        return doc;
    }
}

impl From<Document> for Task {
    fn from(_: Document) -> Self {
        todo!()
    }
}
struct pipeline {}
pub fn push_item(itm: Vec<Task>) -> DatabaseDoc<Document> {
    let transform = itm
        .into_iter()
        .map(|t| {
            let dc = Document::from(t);
            return dc;
        })
        .collect::<Vec<Document>>();

    let push_stage = doc! {
        "$set":{
            "$push":transform,
        }
    };
    return DatabaseDoc::create(push_stage);
}

pub fn create_pipelines() {}

#[test]
fn test() {
    let cmt = Taskdate::new_local();
    let new = Task {
        name: "opa".to_string(),
        created_at: cmt.clone(),
        description: "oaaaa".to_string(),
        completed_at: None,
        completion: false,
        modified_at: cmt,
    };
    let mut abr = Vec::new();
    for _ in 0..10 {
        abr.push(new.clone())
    }
    let success = push_item(abr);
    println!("Sucess wrapper {:?}", success);
}
