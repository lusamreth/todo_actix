use super::*;
use crate::port::error::*;
use crate::port::todo_serv::Taskport;

type DeletionRes<T> = UsecaseRes<Output<T>, PortException>;

pub async fn execute(repo: impl Taskport, id: &str) -> DeletionRes<bool> {
    dbg!(id);
    let del = repo.delete_task(id).await;
    // dbg!(del.clone());
    match del {
        Ok(bool) => Ok(Output {
            payload: Some(bool),
        }),
        Err(err) => Err(err),
    }
}

pub async fn multiple_execute(repo: impl Taskport,ids:Vec<String>) -> DeletionRes<i64>{
    dbg!(&ids);
    let mul_exec = repo.delete_many_task(ids).await;
    match mul_exec {
        Ok(itm) => Ok(Output{
            payload:Some(itm)
        }),
        Err(err) => Err(err)
    }
}