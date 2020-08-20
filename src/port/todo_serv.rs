use super::error::PortException;
use crate::domain::todolist::{list::Todolist, task::Task};
use async_trait::async_trait;
use mongodb::bson::Document;
use super::io::*;

#[allow(type_alias_bounds)]
#[allow(dead_code)]
pub type PortRes<T: Send + 'static> = Result<T, PortException>;
pub type MultipleExceptions = Vec<PortException>;
pub type BundlePortRes<T> = Result<Vec<T>, MultipleExceptions>;

#[async_trait(?Send)]
pub trait Todolistport {
    async fn find_list(&self, id: &str) -> PortRes<Todolist>;
    async fn create_list(&self, actor_input: Todolist) -> PortRes<String>;
    async fn update_list<T: serde::Serialize>(&self, id: &str, new_entity: T) -> PortRes<()>;
    async fn delete_list(&self, id: &str) -> PortRes<()>;
    // async fn append_to_list(&self,input:Task,todo_id:&str) -> PortRes<String>;
}

#[async_trait(?Send)]
pub trait AggregationService{
    async fn merge_task_list<T:serde::Serialize,R>(&self,pipes:Vec<T>) -> BundlePortRes<JoinedOutput>;
}
pub trait BsonConvertor {
    fn to_bson(&self) -> Document;
}

#[async_trait(?Send)]
pub trait Taskport {
    async fn find_task(&self, id: &str) -> PortRes<Task>;
    async fn create_task(&self, task_input: Task) -> PortRes<String>;
    async fn delete_task(&self, id: &str) -> PortRes<bool>;
    async fn update_task(&self, id: &str, new_document: Task) -> PortRes<()>;
    async fn list_all(&self) -> BundlePortRes<Task>;
}

type FutureGateway<T> = std::pin::Pin<Box< dyn futures::Future<Output = T>>>;
pub type GatewayFactory<T> = Box<dyn FnOnce() -> FutureGateway<T>>;