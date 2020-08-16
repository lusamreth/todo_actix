use crate::db::Db;
use crate::domain::resporitory_interface::{self, ITodoresp};
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Bson};
use mongodb::options;
use resperror::DBERROR;
use resporitory_interface::*;
use std::time::Duration;

#[async_trait(?Send)]
impl ITodoresp for Db {
    async fn find_todo(&self, id: Todoid<'_>) -> DocRes {
        let query = doc! {"_id":id};
        let find_cursor = self.collection.find_one(query, None).await;
        match find_cursor {
            Ok(find) => Ok(find),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
        }
    }

    async fn delete(&self, id: Todoid<'_>) -> DeleteRes {
        let query = doc! {"id":id};
        let common_write_con = options::WriteConcern::builder()
            .w_timeout(Duration::from_millis(200))
            .build();
        let del_opt = options::DeleteOptions::builder()
            .write_concern(common_write_con)
            .build();
        let del_cursor = self.collection.delete_one(query, del_opt).await;
        match del_cursor {
            Ok(del) => Ok(del),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
        }
    }
    async fn clear_all(&self) -> DeleteRes {
        let long_tm = Duration::from_millis(400);
        let query = doc! {}; // null
        let common_write_con = options::WriteConcern::builder().w_timeout(long_tm).build();
        let clear_opt = options::DeleteOptions::builder()
            .write_concern(common_write_con)
            .build();
        let del_cursor = self.collection.delete_many(query, clear_opt).await;
        match del_cursor {
            Ok(cr) => Ok(cr),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
        }
    }
    async fn insert_todo<T: serde::Serialize>(&self, entity: T) -> InsertRes {
        let new_doc = to_bson(&entity).expect("cannot convert entity to document!");
        if let Bson::Document(new_doc) = new_doc {
            let create_cur = self.collection.insert_one(new_doc, None).await;
            match create_cur {
                Ok(cr) => Ok(cr),
                Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
            }
        } else {
            Err(DBERROR::Bson(Some(String::from(
                "Invalid bson conversion type in creation of todo!",
            ))))
        }
    }
    async fn update_todo<T: serde::Serialize>(&self, new_entity: T, id: Todoid<'_>) -> UpdateRes {
        let new_doc = to_bson(&new_entity).expect("cannot convert entity to document!");
        let query = doc! {"_id":id};
        if let Bson::Document(update_doc) = new_doc {
            let clt_cursor = self
                .collection
                .update_one(
                    query,
                    doc! {
                        "$set":update_doc
                    },
                    None,
                )
                .await;
            match clt_cursor {
                Ok(cr) => Ok(cr),
                Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
            }
        } else {
            Err(DBERROR::Bson(Some(String::from(
                "Invalid bson conversion during update!",
            ))))
        }
    }
    async fn find_all(&self) -> resporitory_interface::BulkRes<DocRes> {
        todo!()
    }
}
