use crate::db::Db;
use crate::domain::resporitory_interface;
use async_trait::async_trait;
use mongodb::{
    bson::{doc, to_bson, Bson},
    options,
};
use resporitory_interface::{DeleteRes, DocRes, InsertRes, Itaskresp, Taskid, UpdateRes,resperror};
use resperror::DBERROR;

#[async_trait(?Send)]
impl Itaskresp for Db {
    async fn insert_task<T>(&self, entity: T) -> InsertRes where T:serde::Serialize {
        let insert_opt = None;
        let new_doc = to_bson(&entity).expect("msg");
        if let Bson::Document(nd) = new_doc{
            match self.collection.insert_one(nd, insert_opt).await{
                Ok(col) => Ok(col),
                Err(mdberr) => Err(DBERROR::Mongodb(mdberr))
            }
        }else{
            Err(DBERROR::Bson(None))
        }
    }
    async fn update_task<T: serde::Serialize>(&self, new_entity: T, id: Taskid<'_>) -> UpdateRes {
        let to_doc = to_bson(&new_entity).unwrap();
        if let Bson::Document(doc) = to_doc {
            let cursor = self
                .collection
                .update_one(doc! {"_id":id}, doc, None)
                .await;
            match cursor {
                Ok(ptr) => Ok(ptr),
                Err(uerr) => Err(DBERROR::Mongodb(uerr)),
            }
        } else {
            Err(DBERROR::Bson(None))
        }
    }
    async fn delete_task(&self, id: &str) -> DeleteRes {
        let query = doc! {"_id":id};
        let del_cursor = self.collection.delete_one(query, None).await;
        match del_cursor {
            Ok(delc) => Ok(delc),
            Err(del_err) => Err(DBERROR::Mongodb(del_err))
        }
    }
    async fn clear_all_task(&self) -> DeleteRes {
        let query = doc! {};
        let clt_cursor = self.collection.delete_many(query, None).await;
        match clt_cursor{
            Ok(cr) => Ok(cr),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err))
        }
    }
    async fn find_task(&self, id: Taskid<'_>) -> DocRes {
        let query = doc! {"_id":id};
        let proiritized_opt = options::FindOneOptions::builder()
            .read_concern(options::ReadConcern::available())
            .max_time(std::time::Duration::from_millis(200))
            .build();

        let cursor = self.collection.find_one(query, proiritized_opt).await;
        match cursor{
            Ok(curs) => Ok(curs),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err))
        }
    }
}
