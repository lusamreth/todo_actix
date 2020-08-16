use super::*;
use crate::domain::todolist::task::Task;
use crate::port::error::*;
use crate::port::todo_serv::Taskport;

type Insertid = String;
type CreationRes = UsecaseRes<Output<Insertid>,PortException>;

pub async  fn execute(repo:impl Taskport,name:String,desc:String) -> CreationRes{
    let new_task = Task::new(name,desc);
    match new_task {
        Ok(task) => {
            let create = repo.create_task(task).await;
            match create {
                Ok(id) => {
                    Ok(Output {payload:Some(id)})
                },
                Err(e) => Err(e)
            }
        }
        Err(bussiness_err) => {
            let exception = PortError::External(bussiness_err).domain_err();
            Err(exception)
        }
    }
}