use crate::db::Db;
use crate::domain::resporitory_interface::{self, ITodoresp};
use async_trait::async_trait;
use mongodb::bson::{doc, to_bson, Bson, Document};
use mongodb::options;
use resperror::DBERROR;
use resporitory_interface::*;
use std::time::Duration;
use futures::StreamExt;

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
                    update_doc,
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
    async fn find_all(&self) -> resporitory_interface::BulkRes<Document> {
        let query = doc! {};
        let res = self.collection.find(query,None).await;
        let mut err_acc = Vec::new();
        match res {
            Ok(mut cursor) => {
                let mut doc_vec = Vec::new();
                while let Some(doc) = cursor.next().await{
                    match doc {
                        Ok(doc) => {
                            doc_vec.push(doc);
                        }
                        Err(doc_err) => {
                            let msg = format!("Having error while fetching document! \n details : {:#?}",doc_err);
                            let db_er = DBERROR::Operation(msg);
                            err_acc.push(db_er);
                        }
                    }
                }
                if err_acc.len() > 0 {
                    Err(err_acc)
                }else{
                    Ok(doc_vec)
                }
            }
            Err(curs_err) => {
                err_acc.push(DBERROR::Mongodb(curs_err));
                Err(err_acc)
            }
        }
    }

    async fn aggregate<T:serde::Serialize>(&self,pipeline:Pipeline<T>) -> BulkRes<Document>{
        let query = doc! {};
        let mut doc_pipe = Vec::new();
        let mut err_acc = Vec::new();
        
        pipeline.into_iter().for_each(|doc|{
            let bson_opt = mongodb::bson::to_bson(&doc);
            match bson_opt {
                Ok(bson) => {
                    if let Bson::Document(doc) = bson{
                        doc_pipe.push(doc);
                    }
                }
                Err(bson_err) => {
                    let msg=  Some(format!("Parsing bson to document error! wrong format \n{}!",bson_err));
                    err_acc.push(DBERROR::Bson(msg));
                }
            }
        });

        let aggro = self.collection.aggregate(doc_pipe, None).await;
        match aggro {
            Ok(mut cursor) => {
                let mut docs = Vec::new();
                while let Some(doc) = cursor.next().await{
                    match doc{
                        Ok(doc) => {
                            docs.push(doc)
                        }
                        Err(err_doc) => {
                            let msg = format!("Error Ocurred while aggregating data!\n details : {:#?}",err_acc);
                            let db_err = DBERROR::Operation(msg);
                            err_acc.push(db_err)
                        }
                    }
                }
                if err_acc.len() > 0 {
                    Err(err_acc)
                }else{
                    Ok(docs)
                }
            }
            Err(curs_err) => {
                err_acc.push(DBERROR::Mongodb(curs_err));
                Err(err_acc)
            }
        }
    }
}
