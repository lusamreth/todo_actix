use crate::{
    domain::todolist::{list, task::Task, task_date::Taskdate},
    port::io::{DatabaseDoc, JoinedOutput},
};
use mongodb::bson::{doc, Document,oid};
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
        let oid = oid::ObjectId::with_string(&tk.task_id).unwrap();
        let doc = doc! {
            "task_id":oid,
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
                "tasks":td.task_store.tasks,
                "count":num
            },
            "created_at":cta,
            "due_date":due_date
        };
        return new_doc;

    }
}


pub fn push_item(itms: Vec<String>) -> DatabaseDoc<Document> { 
    let bs_arr= itms.into_iter().map(|id| oid::ObjectId::with_string(&id).unwrap()).collect::<Vec<oid::ObjectId>>();
    
    let push_stage = doc! {
        "$push":{
            "task_store.tasks":{
                "$each":bs_arr
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
        "from":a.as_str(),
        "localField":b.as_str(),
        "foreignField":c.as_str(),
        "as":d.as_str()
        }
    };
    dbg!(look_up_pipe.clone());
    DatabaseDoc::create(look_up_pipe)
}

impl From<Document> for Task{
    fn from(task_doc: Document) -> Self {        
        let get = |name:&str| {
            if let Bson::String(i) = task_doc.get(name).unwrap().to_owned(){
                return i;
            }else{
                panic!("bad format!")
            }
        };
        dbg!(&task_doc);
        let mdf = get("modified_at");
        let ctf = get("created_at");
        let mut new = Task::new(get("name"), get("description")).expect("Server internet bad filtering!");
        new.insert_created_at(Taskdate::from_string(ctf));
        new.insert_modified_at(Taskdate::from_string(mdf));
        let mut cmpt = None;
        if let Bson::String(i) = task_doc.get("completed_at").unwrap(){
            cmpt = Some(Taskdate::from_string(i.clone()))
        }
        new.completed_at = cmpt;
        new.task_id = get("task_id");
        return new;
    }
}

impl From<Document> for JoinedOutput {
    fn from(bulk_res: Document) -> Self {
        println!("doc {:#?}",bulk_res.clone());
        let to_td = move |date: &Bson| {
            if let Bson::String(date_str) = date.clone(){
                return Taskdate::from_string(date_str);
            }else{
                Taskdate::new_local()
            }
        };
        //list_name use it! instead of name
        let name = bulk_res.get("list_name").unwrap();
        let modifed_at = to_td(bulk_res.get("modified_at").unwrap());
        let created_at = to_td(bulk_res.get("created_at").unwrap());
        let due_date = to_td(bulk_res.get("due_date").unwrap());
        let dued = bulk_res.get("dued").unwrap().as_bool().unwrap();
        // use task_store instead of tk
        // let doc = bulk_res.get("task_store").unwrap().as_document().unwrap();

        let items = if let Bson::Array(store) = bulk_res.get("task_doc").unwrap().clone(){
            let vec_str = store.into_iter().map(|bid| {
                dbg!(&bid);
                let t:Task = Task::from(bid.as_document().unwrap().to_owned());
                return t;
            }).collect::<Vec<Task>>();
            vec_str
        }else{
            Vec::new()
        };
        let done = bulk_res.get("done").unwrap().as_bool().unwrap();
        let progress = bulk_res.get("progress").unwrap().as_f64().unwrap() as f32;

        return {
            JoinedOutput {
                task_id:"sandklasnd".to_string(),
                created_at,
                modifed_at: Some(modifed_at),
                done,
                progress,
                list_name: name.to_string(),
                due_date: Some(due_date),
                dued,
                task_store:items,
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
    dbg!(task_col);
    let oem = Matchstage {
        from: task_col.to_string(),
        localfield: "task_store.tasks".to_string(),
        foreignfield: "task_id".to_string(),
        as_domain: "task_doc".to_string(),
    };
    match_pipeline(oem)
}

pub fn pull_task_linkage(tasks_id:Vec<String>) -> DatabaseDoc<Document>{
    let id_bson = Bson::Array(tasks_id.into_iter().map(|task_id|{
        Bson::String(task_id)
    }).collect::<Vec<Bson>>());
    let entity = doc! {
        "$pull":{
            "task_store.tasks":{
                "task_id":{
                    "$in":id_bson
                }
            }
        }
    };
    DatabaseDoc::create(entity)
}