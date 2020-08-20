use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct CreationOutput{
    pub(crate) upsert : bool,
    pub(crate) message : String
}
#[derive(Debug,Serialize,Deserialize)]
pub struct FindingOuput<T>{
    pub id:String,
    pub(crate) task:T
}

#[derive(Debug,Serialize,Deserialize)]
pub struct DeletionOutput{
    pub(crate) acknowledgement:bool,
    pub(crate) message:String
}

use crate::http::resources::utils::OutputJson;

impl <T: Serialize> DefaultOutput<String> for FindingOuput<T> {}
impl DefaultOutput<CreationRelation> for CreationOutput {
    fn json_output(self,id:Option<String>) -> OutputJson<Self,CreationRelation>{
        let relationship = CreationRelation{
            created_task:Chainlink::new().self_link(format!("/task/{}",id.clone().unwrap_or("".to_string())).as_str())
        };
        OutputJson{
            data_type: String::from("Task"),
            id,
            attributes:self,
            relationship:Some(relationship),
            
        }
    }
}
use crate::http::resources::utils::{DefaultOutput, Chainlink};


#[derive(Debug,Serialize,Deserialize)]
pub struct CreationRelation{
    created_task:Chainlink
}

impl DefaultOutput<String> for DeletionOutput {}
