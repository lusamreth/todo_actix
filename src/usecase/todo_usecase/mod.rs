use crate::port::todo_serv::Todolistport;
// type FactoryResult<I,O> = Box<dyn FnOnce(I) -> Pin<Box<dyn Future<Output=O>>>>;
// type FactoryDoubleArg<'a,I1,I2,O> = Box<dyn Fn(I1,I2) -> Pin<Box<dyn Future<Output=O> + 'a>>>;
use crate::usecase::*;
pub mod append_list;
pub mod create_list;
pub mod delete_list;
pub mod list_tasks;
pub mod search_list;
pub mod remove_task;
//Docs  or side-note?:
/*
// Don't Leak responsibility of presenter! Such as returning message
*/
