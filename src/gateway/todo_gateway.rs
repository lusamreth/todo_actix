
use crate::domain;
use domain::resporitory_interface::ITodoresp;
use json::{JsonValue,object};
use async_trait::async_trait;
use super::Gateway;

use crate::port::todo_serv::{Todolistport,PortRes};
use domain::todolist::list::Todolist;
use crate::db::Db;

async fn todo_col() -> Db{
    Db::fetch_collection("env+var".to_string()).await
}

#[async_trait(?Send)]
impl <'a> Todolistport<'a> for Gateway<'_>{
    async fn update_list(id:&str,new_entity:Todolist) -> PortRes<()> {
        let db = todo_col().await;
        let patcher = db.update_todo(new_entity, id).await;
        match patcher{
            Ok(u_res) => {
                let match_count = u_res.matched_count.to_string();
                let confirmed = u_res.modified_count.to_string();
                let json_ct = object! { matched:match_count,confirmation : confirmed };
                println!("{:#?}",json_ct);
                if u_res.matched_count < 0{
                    // cannot find doc!
                    Err(String::from("No document match the index!"))
                }else if u_res.modified_count < 0{
                    // failed to write updates
                    Err(String::from("confirmation of updating failed"))
                }else{
                    Ok(())
                }
            }
            Err(mdb_err) => Err(mdb_err.get_string())
        }
    }

    async fn delete_list(id:&str) -> PortRes<()> {
        let db = todo_col().await;
        let deletion = db.delete(id).await;
        match deletion{
            Ok(ct) => {
                let string_ct = ct.deleted_count.to_string();
                let json_ct = object! { mdb_json : string_ct };
                println!("{:#?}",json_ct);
                
                if ct.deleted_count < 0{
                    // deletion failed due to any err!
                    Err("Deletion operation does not take into affect!".to_string())
                }else{
                    Ok(())
                }
            },
            Err(mdb_err) => Err(mdb_err.get_string())
        }
    }
    
    async fn create_list(actor_input : Todolist) -> PortRes<()> {
        let db = todo_col().await;
        let creation = db.insert_todo(actor_input).await;
        match creation {
            Ok(ct) => {
                let string_ct = ct.inserted_id.to_string();
                let json_ct = object! { mdb_json : string_ct };
                println!("{:#?}",json_ct);
                Ok(())
            },
            Err(mdb_err) => Err(mdb_err.get_string())
        }
    } 

    async fn find_list(self,id:&str) -> PortRes<JsonValue> {
        let db = (self.col)().await;
        let tdl = db.find_todo(id).await;
        match tdl{
            Ok(doc) => {
                let doc_json = match doc {
                    Some(avialable) => {
                        let to_string = avialable.to_string();
                        object! {list:to_string}
                    },
                    None => {
                        let empty = String::new();
                        object! {list:empty}
                    }
                };
                return Ok(doc_json);
            },
            Err(mdb_err) => {
                let error_str = mdb_err.get_string();
                return Err(error_str);
            }
        }
    }
}
