use serde::{Deserialize,Serialize};
use crate::http::resources::utils::{Chainlink,DefaultOutput,OutputJson};
// The output of apis
#[derive(Debug,Serialize,Deserialize)]
struct CreationJson<T>{
    details:T,
    creation_id : String,
    timestamp:i64
}

#[derive(Debug,Serialize,Deserialize)]
struct CreationRelation{
    entity:Alltask,
    data:EntType
}

#[derive(Debug,Serialize,Deserialize)]
struct EntType{
    ent_type:String,
    id:String
}

#[derive(Debug,Serialize,Deserialize)]
struct Alltask{
    task:Chainlink
}

#[derive(Debug,Serialize,Deserialize)]
enum DelType{
    //COMPLETE
    HARD,
    //CACHED
    SOFT
}


#[derive(Debug,Serialize,Deserialize)]
struct DeletionJson{
    acknowledegement:bool,
    deletion_type:DelType,
    deleted_date:String
}


#[derive(Debug,Serialize,Deserialize)]
struct EditionJson<T>{
    modified_date:String,
    acknowledegement:bool,
    details:T
}


#[derive(Debug,Serialize,Deserialize)]
struct FinderJson<T>{
    todolist:T
}
#[derive(Debug,Serialize,Deserialize)]
struct FinderRelation{  
    found_list:Chainlink
}

impl <T:Serialize> DefaultOutput<FinderRelation> for FinderJson<T>{
    fn json_output(self,id:Option<String>) ->  OutputJson<Self,FinderRelation>{
        let relation = FinderRelation {
            found_list:Chainlink::new().other_link(format!("/tasks").as_str())
        };
        OutputJson{
            data_type: String::from("Task"),
            id,
            attributes:self,
            relationship: Some(relation),
            
        }
    }
}