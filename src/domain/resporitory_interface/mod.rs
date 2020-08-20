use async_trait::async_trait;
use mongodb::{bson::Document, results};
use serde::Serialize;
pub mod resperror;
use resperror::DBERROR;
#[async_trait(?Send)]
pub trait Itaskresp {
    async fn insert_task<T>(&self, entity: T) -> InsertRes
    where
        T: Serialize;
    async fn update_task<T>(&self, new_entity: T, id: Taskid<'_>) -> UpdateRes
    where
        T: Serialize;
    async fn delete_task(&self, id: Taskid<'_>) -> DeleteRes;
    async fn clear_all_task(&self) -> DeleteRes;
    async fn find_task(&self, id: Taskid<'_>) -> DocRes;
    async fn find_all(&self) -> BulkRes<Document>;
}

#[async_trait(?Send)]
pub trait ITodoresp {
    async fn insert_todo<T>(&self, entity: T) -> InsertRes
    where
        T: Serialize;
    async fn update_todo<T>(&self, new_entity: T, id: &str) -> UpdateRes
    where
        T: Serialize;
    async fn delete(&self, id: Todoid<'_>) -> DeleteRes;
    async fn clear_all(&self) -> DeleteRes;
    async fn find_all(&self) -> BulkRes<Document>;
    async fn find_todo(&self, id: Todoid<'_>) -> DocRes;
    async fn aggregate<T:Serialize>(&self,pipeline:Pipeline<T>) -> BulkRes<Document>;
}

pub type Pipeline<T> = Vec<T>;
pub type Taskid<'a> = &'a str;
pub type Todoid<'a> = &'a str;
pub type InsertRes = Result<results::InsertOneResult, DBERROR>;
pub type UpdateRes = Result<results::UpdateResult, DBERROR>;
pub type DeleteRes = Result<results::DeleteResult, DBERROR>;
pub type DocRes = Result<Option<mongodb::bson::Document>, DBERROR>;
pub type BulkRes<T> = Result<Vec<T>, Vec<DBERROR>>;
