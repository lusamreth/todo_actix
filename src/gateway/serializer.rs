use crate::{
    domain::todolist::{list, task::Task, task_date::Taskdate},
    port::io::{DatabaseDoc, JoinedOutput},
};
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
// abstraction layer built to remove dependencies

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
            "description":tk.description,
            "created_at":tk.created_at.to_string(),
            "modified_at":mdf_str,
            "completion" : tk.completion
        };
        return doc;
    }
}



impl From<list::Todolist> for Document{
    fn from(td:list::Todolist) -> Self{
        let mdf = td.modifed_at.to_string();
        let cta = td.created_at.to_string();
        let due_date = match td.due_date{
            Some(tkat) => Bson::String(tkat.to_string()),
            None => Bson::Null,
        };
        let num = Bson::Int32(td.task_store.count as i32);
        let new_doc = doc! {
            "list_name":td.list_name,
            "done":td.done,
            "modified_at":mdf,
            "dued":td.dued,
            "progress":td.progress,
            "task_store":{
                "store":td.task_store.tasks,
                "count":num
            },
            "created_at":cta,
            "due_date":due_date
        };
        return new_doc;

    }
}

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
            "task_store.tasks":{
                "$push":transform,
            }
        }
    };
    return DatabaseDoc::create(push_stage);
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Matchstage {
    from: String,
    localfield: String,
    foreignfield: String,
    as_domain: String,
}

pub fn match_pipeline(match_stage: Matchstage) -> DatabaseDoc<Document> {
    let Matchstage {
        from: a,
        localfield: b,
        foreignfield: c,
        as_domain: d,
    } = match_stage;
    let look_up_pipe = doc! {
        "$lookup":{
        "from":a,
        "localField":b,
        "foreignField":c,
        "as":d
        }
    };
    DatabaseDoc::create(look_up_pipe)
}

impl From<Document> for JoinedOutput {
    fn from(bulk_res: Document) -> Self {
        let to_td = move |date: &Bson| {
            let task_date: Taskdate =
                mongodb::bson::from_bson(date.clone()).expect("format error!");
            return task_date;
        };
        //list_name use it! instead of name
        let name = bulk_res.get("name").unwrap();
        let modifed_at = to_td(bulk_res.get("modified_at").unwrap());
        let created_at = to_td(bulk_res.get("created_at").unwrap());
        let due_date = to_td(bulk_res.get("due_date").unwrap());
        let dued = bulk_res.get("dued").unwrap().as_bool().unwrap();
        // use task_store instead of tk
        let task_store =
            mongodb::bson::from_bson(bulk_res.get("tk").unwrap().clone()).expect("failed");
        let done = bulk_res.get("done").unwrap().as_bool().unwrap();
        let progress = bulk_res.get("progress").unwrap().as_f64().unwrap() as f32;

        return {
            JoinedOutput {
                created_at,
                modifed_at: Some(modifed_at),
                done,
                progress,
                list_name: name.to_string(),
                due_date: Some(due_date),
                dued,
                task_store,
            }
        };
    }
}
/*
TODO_LIST_COLLECTION
TODO_TASK_COLLECTION
*/

// from a list perspective!
pub fn list_task_linkage() -> DatabaseDoc<Document> {
    let task_col = dotenv!("TODO_TASK_COLLECTION");
    let oem = Matchstage {
        from: task_col.to_string(),
        localfield: "task_store.store".to_string(),
        foreignfield: "_id".to_string(),
        as_domain: "task_store".to_string(),
    };
    match_pipeline(oem)
}
