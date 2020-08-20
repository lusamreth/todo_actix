use serde::{Serialize,Deserialize};
use crate::usecase::task_usecase::edit_task::UpdateTaskField;

#[derive(Serialize,Deserialize,Clone)]
pub struct MakeTaskJson{
    pub name:String,
    pub description:String
}

#[derive(Serialize,Deserialize,Clone)]
pub struct UpdateTaskJson{
    pub target_id : String,
    pub completion:CompletionState,
    pub fields:UpdateTaskField

}
#[derive(Serialize,Deserialize,Clone)]
pub struct CompletionState{
    pub done:bool,
    pub complete_date:Option<String>,
    pub complete_time:Option<String>
}

impl Clone for UpdateTaskField{
    fn clone(&self) -> Self {
        let mut new = UpdateTaskField::new();
        new.name = self.name.clone();
        new.desc = self.desc.clone();
        new.completion = self.completion;
        return new;
    }
}

//Operations!
#[derive(Debug,Serialize,Deserialize)]
pub struct FindTaskJson<T:Serialize>{
    pub(crate) payloads:Vec<T> 
}

//Creation