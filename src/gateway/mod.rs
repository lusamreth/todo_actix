mod todo_gateway;
use crate::db::Db;
use std::future::Future;
use std::pin::Pin;
pub mod serializer; // no public
                    //take out!
pub mod task_gateway;
#[allow(dead_code)]
pub struct Gateway {
    col: TodoCol,
}
// type TodoCol = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Db>>>>;
type TodoCol = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Db> + 'static>>>;

impl Gateway {
    #[allow(non_snake_case, dead_code)]
    pub async fn establish<Fx: 'static>(db_col: &str, envFn: Fx) -> Gateway
    where
        Fx: FnOnce(&str) -> String,
    {
        let param = envFn(&db_col);
        Gateway {
            col: Box::new(move || {
                let pdb = Db::fetch_collection(param.clone());
                Box::pin(pdb)
            }),
        }
    }
}
#[allow(dead_code)]
fn fetch_env(arg: &str) -> String {
    match dotenv::var(arg) {
        Ok(var) => var,
        Err(var_err) => panic!("No such variable {} exists \n details : {}", arg, var_err),
    }
}
