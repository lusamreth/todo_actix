use async_trait::async_trait;
use mongodb::bson::Document;
use crate::driver::task_drv;
use crate::domain::resporitory_interface;
use crate::db::Db;
use resporitory_interface::{Itaskresp,ITodoresp,resperror::DBERROR,Todoid};
use crate::domain::todolist::{task::Task,list::Todolist};

#[allow(type_alias_bounds)]
#[allow(dead_code)]
pub type PortRes<T:Send + 'static> = Result<T,String>;

#[async_trait(?Send)]
pub trait Todolistport<'a>{
    async fn find_list(self,id:&str) -> PortRes<json::JsonValue>;
    async fn create_list(actor_input:Todolist) -> PortRes<()>;
    async fn update_list(id:&str,new_entity:Todolist) -> PortRes<()>;
    async fn delete_list(id:&str) -> PortRes<()>;
}

enum PortError {
    Internal(String),
    External(String)
}
impl PortError{
    pub fn emit_internal(&self){
        if let PortError::Internal(in_err) = self{
            println!("Internal Error from Driver entity!");
            println!("Details : \n");
            println!("{}",in_err);
            panic!("Error internal!");
        }else{
            println!("This error is external! go to transfer!")
        }
    }
    pub fn transfer(&self) -> String{
        if let PortError::External(ex_err) = self{
            return ex_err.to_owned();
        }else{
            panic!("This error is not transferable!")
        }
    }
    fn bundle_emit(buffer:Vec<Self>){
        buffer.iter().for_each(|each_err| {
            println!("Emiting from bundles!");
            if let PortError::Internal(_) = each_err{
                each_err.emit_internal();
            }else {
                each_err.transfer();
            }
        })
    }
}
// polymorphism : the provision of a single interface to multiple entites with different concrete type ;
/*  
    interface with function draw! <Trait-object>
    trait Component{
        fn draw() -> String;
    }
    // concrete types: 
    struct pen{..}
    struct pencil{..}

    // implementations
    impl Component for pen{
        fn draw() -> String{..}
    }
    impl Component for pencil{
        fn draw() -> String{..}
    }
    // this struct stores a vector of trait object!
    struct Comp{
        components:Vec<Component>
    }
    impl Comp{
        fn run(&self){
            self.components.iter().foreach(|cp| {
                // calling the interface's function , the runtime will peform polymorphic operations!
                // the vtable's pointer in trait object will point to the vtable of each concrete implementation
                // than vtable will then point to the actual implementation of that methods;
                cp.draw()
            })
        }
    }
    *Visualization :
    =>This lookup transformation will be executed at runtime only since it will know where those trait object's
    method will be called during application running;
    /*********************/
    |Data-object| <--- |Trait objects|
    |pen's members|    |vtb's ptr| ---> |pen's vtable| -> |actual draw impl|
    |pencil's members| |vtb's ptr| ---> |pencil's vtable| -> |actual draw impl|
    /*********************/
    Note : vtable has other components such as size and align!;In rust the drop function is automatically impl;
    So the table will also include that(drop's ptr) as well!;
*/