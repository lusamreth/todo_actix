use crate::db::Db;
use crate::domain::resporitory_interface;
use async_trait::async_trait;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, to_bson, Bson},
    options,
};
use resperror::DBERROR;
use resporitory_interface::*;

#[async_trait(?Send)]
impl Itaskresp for Db {
    async fn insert_task<T>(&self, entity: T) -> InsertRes
    where
        T: serde::Serialize,
    {
        let insert_opt = None;
        let new_doc = to_bson(&entity).expect("msg");
        if let Bson::Document(nd) = new_doc {
            match self.collection.insert_one(nd, insert_opt).await {
                Ok(col) => Ok(col),
                Err(mdberr) => Err(DBERROR::Mongodb(mdberr)),
            }
        } else {
            Err(DBERROR::Bson(None))
        }
    }
    async fn update_task<T: serde::Serialize>(&self, new_entity: T, id: Taskid<'_>) -> UpdateRes {
        match ObjectId::with_string(id) {
            Ok(objid) => {
                let to_doc = to_bson(&new_entity).unwrap();
                if let Bson::Document(doc) = to_doc {
                    let cursor = self
                        .collection
                        .update_one(doc! {"task_id":objid}, doc, None)
                        .await;
                    match cursor {
                        Ok(ptr) => Ok(ptr),
                        Err(uerr) => Err(DBERROR::Mongodb(uerr)),
                    }
                } else {
                    Err(DBERROR::Bson(None))
                }
            }
            Err(_) => Err(DBERROR::Bson(None)),
        }
    }
    async fn delete_task(&self, id: &str) -> DeleteRes {
        match ObjectId::with_string(id) {
            Ok(objid) => {
                let query = doc! {"task_id":objid};
                let del_cursor = self.collection.delete_one(query, None).await;
                match del_cursor {
                    Ok(delc) => Ok(delc),
                    Err(del_err) => Err(DBERROR::Mongodb(del_err)),
                }
            }
            Err(del_err) => Err(DBERROR::Bson(Some(del_err.to_string()))),
        }
    }
    async fn delete_many(&self,queries:Vec<String>) -> DeleteRes {
        let bq = queries.into_iter().map(|itm| Bson::String(itm)).collect::<Vec<Bson>>();
        let query = doc! {
            "task_id":{
                "$in":bq
            }
        };
        let clt_cursor = self.collection.delete_many(query, None).await;
        match clt_cursor {
            Ok(cr) => Ok(cr),
            Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
        }
    }
    async fn find_task(&self, id: Taskid<'_>) -> DocRes {
        match ObjectId::with_string(id) {
            Ok(id) => {
                let query = doc! {"task_id":id};
                let proiritized_opt = options::FindOneOptions::builder()
                    .read_concern(options::ReadConcern::available())
                    .max_time(std::time::Duration::from_millis(200))
                    .build();

                let cursor = self.collection.find_one(query, proiritized_opt).await;
                match cursor {
                    Ok(curs) => Ok(curs),
                    Err(cr_err) => Err(DBERROR::Mongodb(cr_err)),
                }
            }
            Err(_) => Err(DBERROR::Bson(Some(
                "bad string id! Cannot parse to obj!".to_string(),
            ))),
        }
    }
    async fn find_all(&self) -> BulkRes<mongodb::bson::Document> {
        let query = doc! {};
        let finder_curs = self.collection.find(query, None).await;
        let mut err_acc = Vec::new();
        match finder_curs {
            Ok(mut cursor) => {
                let mut emdvec = Vec::new();
                while let Some(doc_res) = cursor.next().await {
                    match doc_res {
                        Ok(doc) => emdvec.push(doc),
                        Err(doc_er) => {
                            let err_format = DBERROR::Operation(format!(
                                "Error while fetching tasks! Kind:{}",
                                doc_er.kind
                            ));
                            err_acc.push(err_format)
                        }
                    }
                }
                return Ok(emdvec);
            }
            Err(curs_err) => {
                err_acc.push(DBERROR::Mongodb(curs_err));
                Err(err_acc)
            }
        }
    }
}
