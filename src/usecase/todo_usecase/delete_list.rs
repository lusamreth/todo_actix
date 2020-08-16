use super::Todolistport;
use super::*;
use crate::port::error::*;
type DeletionRes<T> = UsecaseRes<Output<T>, PortException>;

async fn execute(db: impl Todolistport, id: String) -> DeletionRes<()> {
    let creation = db.delete_list(id.as_str()).await;
    match creation {
        Ok(_) => Ok(Output { payload: None }),
        Err(err) => Err(err),
    }
}
