use super::Todolistport;
use super::*;
use crate::port::error::*;
type DeletionRes<T> = UsecaseRes<Output<T>, PortException>;
use crate::gateway::serializer;

pub async fn execute(db: impl Todolistport , id: String,tasks_id:Vec<String>) -> DeletionRes<()> {
    let delete_linkage = serializer::pull_task_linkage(tasks_id);
    let creation = db.update_list(id.as_str(),delete_linkage.get_doc()).await;

    match creation {
        Ok(_) => Ok(Output { payload: None }),
        Err(err) => Err(err),
    }
}
