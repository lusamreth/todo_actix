use super::Todolistport;
use super::*;
use crate::domain::todolist::list;
use crate::port::error::*;
use list::Todolist;

type CreationRes<T> = UsecaseRes<Output<T>, PortException>;
type InsertId = String;
async fn execute(db: impl Todolistport, name: String) -> CreationRes<InsertId> {
    let new = Todolist::new(name.as_str());
    let creation = db.create_list(new).await;
    match creation {
        Ok(res) => Ok(Output { payload: Some(res) }),
        Err(err) => Err(err),
    }
}
