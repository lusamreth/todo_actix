use super::Todolistport;
use super::*;
use crate::domain::todolist::task::Task;
use crate::gateway::serializer::push_item;
use crate::port::error::PortException;

#[allow(dead_code)]
type Taskqueue = Vec<Task>;
pub type AppendRes<T> = UsecaseRes<Output<T>, PortException>;

pub async fn execute<'a>(
    gtw: impl Todolistport,
    id: String,
    new_object: Taskqueue,
) -> AppendRes<()> {
    let db = gtw;
    // let task_creation;
    let update_stage = push_item(new_object);
    let creation = db.update_list(&id, update_stage.get_doc()).await;
    match creation {
        Ok(_) => Ok(Output { payload: None }),
        Err(err) => Err(err),
    }
}

// wrr : format!("Document with id {} has been sucessfully modified!",id);
