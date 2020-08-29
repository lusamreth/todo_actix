use super::Gateway;
use crate::domain;
use crate::port::{
    error::{PortError::*, *},
    io::JoinedOutput,
    todo_serv::{AggregationService, BundlePortRes, PortRes, Todolistport},
};
use async_trait::async_trait;
use domain::resporitory_interface::ITodoresp;
use domain::todolist::list::Todolist;
use json::object;
use mongodb::bson::{Bson,Document};

#[async_trait(?Send)]
impl Todolistport for Gateway {
    async fn update_list<T: serde::Serialize>(&self, id: &str, new_entity: T) -> PortRes<()> {
        let db = (self.col)().await;
        let patcher = db.update_todo(new_entity, id).await;
        match patcher {
            Ok(u_res) => {
                let match_count = u_res.matched_count.to_string();
                let confirmed = u_res.modified_count.to_string();
                let json_ct = object! { matched:match_count,confirmation : confirmed };
                println!("{:#?}", json_ct);
                if u_res.matched_count < 0 {
                    // cannot find doc!
                    Err(External("No document match the index!".to_string()).operation_err())
                } else if u_res.modified_count < 0 {
                    // failed to write updates
                    Err(External("confirmation of updating failed".to_string()).operation_err())
                } else {
                    Ok(())
                }
            }
            Err(mdb_err) => panic!(Internal(mdb_err.get_string()).emit_internal()),
        }
    }

    async fn delete_list(&self, id: &str) -> PortRes<()> {
        let db = (self.col)().await;
        let deletion = db.delete(id).await;
        match deletion {
            Ok(ct) => {
                let string_ct = ct.deleted_count.to_string();
                let json_ct = object! { DeleteAck : string_ct };
                println!("{:#?}", json_ct);

                if ct.deleted_count < 0 {
                    // deletion failed due to any err!
                    Err(
                        External("Deletion operation does not take into affect!".to_string())
                            .operation_err(),
                    )
                } else {
                    Ok(())
                }
            }
            Err(mdb_err) => panic!(Internal(mdb_err.get_string()).emit_internal()),
        }
    }

    async fn create_list(&self, actor_input: Todolist) -> PortRes<String> {
        let db = (self.col)().await;

        let creation = db.insert_todo(Document::from(actor_input)).await;
        match creation {
            Ok(ct) => {
                let sct = ct.inserted_id;
                let mut string_ct = String::new();
                if let Bson::ObjectId(stdr) = sct{
                    string_ct.push_str(&stdr.to_string());
                }
                println!("{:#?}", string_ct);
                Ok(string_ct)
            }
            Err(mdb_err) => panic!(Internal(mdb_err.get_string()).emit_internal()),
        }
    }

    async fn find_list(&self, id: &str) -> PortRes<Todolist> {
        let db = (self.col)().await;
        let tdl = db.find_todo(id).await;
        match tdl {
            Ok(doc) => match doc {
                Some(avialable) => {
                    let name = avialable.get("list_name").unwrap().to_string();
                    let output = Todolist::new(&name);
                    return Ok(output);
                }
                None => {
                    let mut ext_err =
                        External(format!("The list with an id of {} is not found!", id));
                    let mut exp = ext_err.operation_err();
                    exp.sub_type = String::from("NOTFOUND");
                    Err(exp)
                }
            },
            Err(mdb_err) => {
                let error_str = PortError::convert(mdb_err);
                Err(error_str)
            }
        }
    }
}

// pub created_at: Taskdate,
// // likely to change
// pub modifed_at: Option<Taskdate>,
// pub done: bool,
// //dynamic
// pub progress: f32,
// pub list_name: String,
// pub due_date: Option<Taskdate>,
// pub dued: bool,
// // dynamic!
// pub task_store: Vec<Task>,

#[async_trait(?Send)]
impl AggregationService for Gateway {
    async fn merge_task_list<T: serde::Serialize>(
        &self,
        pipes: Vec<T>,
    ) -> BundlePortRes<JoinedOutput> {
        let db = (self.col)().await;
        let aggregation = db.aggregate(pipes).await;
        match aggregation {
            Ok(bulk_res) => {
                dbg!(&bulk_res);
                let vjo = bulk_res
                    .into_iter()
                    .map(|res| JoinedOutput::from(res))
                    .collect::<Vec<JoinedOutput>>();
                Ok(vjo)
            }
            Err(bulk_err) => Err(bulk_err
                .into_iter()
                .map(|each_err| PortError::convert(each_err))
                .collect()),
        }
    }
}

// Some key notes !
/*

*/
