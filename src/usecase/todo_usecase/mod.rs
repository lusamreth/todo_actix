use crate::gateway;
use crate::domain::todolist;
use crate::port::todo_serv::Todolistport;
async fn create_list(db:impl Todolistport<'static>) -> Result<json::JsonValue,String>{
    let dbres = db.find_list("apple").await;
    return dbres;
}