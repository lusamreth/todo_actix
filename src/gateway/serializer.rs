use crate::{port::io::{DatabaseDoc,JoinedOutput}, domain::todolist::{self, task_date::Taskdate, task::Task}};
use mongodb::bson::{doc, Document};
use dotenv::dotenv;

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
            "created_at":tk.created_at.to_string(),
            "modified_at":mdf_str,
            "completion" : tk.completion
        };
        return doc;
    }
}

impl From<Document> for Task {
    fn from(task: Document) -> Self {
        todo!()
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
pub struct Matchstage{
    from:String,
    localfield:String,
    foreignfield:String,    
    as_domain:String
}



pub fn match_pipeline(match_stage:Matchstage) -> DatabaseDoc<Document>{
    let Matchstage {
        from:a,
        localfield:b,
        foreignfield:c,    
        as_domain:d
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
        let to_td = move |date:&Bson|{
            let task_date:Taskdate = mongodb::bson::from_bson(date.clone()).expect("format error!");
            return task_date
        };

        let name = bulk_res.get("list_name").unwrap();
        let modifed_at = to_td(bulk_res.get("modifed_at").unwrap());
        let created_at = to_td(bulk_res.get("created_at").unwrap());
        let due_date = to_td(bulk_res.get("due_date").unwrap());
        let dued = bulk_res.get("dued").unwrap().as_bool().unwrap();   
        let task_store = mongodb::bson::from_bson(bulk_res.get("task_store").unwrap().clone()).expect("failed");
        let done = bulk_res.get("done").unwrap().as_bool().unwrap();
        let progress= bulk_res.get("progress").unwrap().as_f64().unwrap() as f32;

        return {
            JoinedOutput{ created_at, modifed_at:Some(modifed_at) , done, progress, list_name:name.to_string(), due_date:Some(due_date), dued, task_store}
        }
    }
}
/*
TODO_LIST_COLLECTION
TODO_TASK_COLLECTION
*/

// from a list perspective!
pub fn list_task_linkage()-> Matchstage {
    let task_col = dotenv!("TODO_TASK_COLLECTION");
    Matchstage{ from: task_col.to_string(), localfield: "_id".to_string(), foreignfield: "_id".to_string(), as_domain: "task_store".to_string()}
}
