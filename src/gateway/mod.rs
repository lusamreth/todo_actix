use std::env;
use dotenv_codegen::dotenv;
mod todo_gateway;
use std::pin::Pin;
use std::future::Future;
use crate::db::Db;
#[allow(dead_code)]
pub struct Gateway<'a>{
    col:TodoCol<'a>
}
// type TodoCol = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Db>>>>;
type TodoCol<'a> = Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Db> + 'a>>>;

impl Gateway<'_>{
    #[allow(non_snake_case)]
    pub async fn establish<Fn>(db_col : &str,envFn:Fn) -> Gateway<'_> where Fn : FnOnce(&str) -> String{
        let env = envFn(db_col);
        Gateway{
            col: Box::new(||{
                Box::pin(Db::fetch_collection(env))
            })
        }
    }
}
fn fetch_env(arg:&str) -> String{
    match dotenv::var(arg){
        Ok(var) => var,
        Err(var_err) => panic!("No such variable {} exists \n details : {}",arg,var_err)
    }
}

pub async fn built_todo_gateway() -> Gateway<'static>{
    let todo = Gateway::establish("todo+col",fetch_env).await;
    return todo;
}
