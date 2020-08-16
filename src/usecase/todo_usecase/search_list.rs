use super::*;
use crate::domain::todolist::list::Todolist;
use crate::port::error::PortException;
use crate::port::todo_serv::Todolistport;

type TaskFinderRes<T> = UsecaseRes<Output<T>, PortException>;

async fn execute(gtw: impl Todolistport, id: &str) -> TaskFinderRes<Todolist> {
    let finder = gtw.find_list(id).await;
    match finder {
        Ok(res) => Ok(Output { payload: Some(res) }),
        Err(port_err) => Err(port_err),
    }
}
