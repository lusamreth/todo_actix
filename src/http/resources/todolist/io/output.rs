use crate::http::resources::utils::{Chainlink, DefaultOutput, OutputJson};
use serde::{Deserialize, Serialize};
// The output of apis
#[derive(Debug, Serialize, Deserialize)]
pub struct CreationJson<T> {
    pub(crate) details: T,
    pub appended_tasks:Vec<String>,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreationRelation {
    entity: Alltask,
    data: EntType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntType {
    ent_type: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alltask {
    task: Chainlink,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DelType {
    //COMPLETE
    HARD,
    //CACHED
    SOFT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletionJson {
    pub acknowledegement: bool,
    pub deletion_type: DelType,
    pub deleted_date: String,
    pub task_count:i64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditionJson<T> {
    modified_date: String,
    acknowledegement: bool,
    details: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinderJson<T> {
    pub(crate) todolist: T,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FinderRelation {
    found_list: Chainlink,
}

impl<T: Serialize> DefaultOutput<FinderRelation> for FinderJson<T> {
    fn json_output(self, id: Option<String>) -> OutputJson<Self, FinderRelation> {
        let relation = FinderRelation {
            found_list: Chainlink::new()
                .other_link(format!("/tasks/{}", id.clone().unwrap_or(String::new())).as_str()),
        };
        OutputJson {
            data_type: String::from("Task"),
            id,
            attributes: self,
            relationship: Some(relation),
        }
    }
}

#[derive(Serialize,Debug)]
pub struct DeleteRelation{
    deleted:Chainlink
}

impl<T: Serialize> DefaultOutput<CreationRelation> for CreationJson<T> {}
impl DefaultOutput<DeleteRelation> for DeletionJson {
    fn json_output(self, id: Option<String>) -> OutputJson<Self, DeleteRelation> {
        let relation = DeleteRelation {
            deleted: Chainlink::new()
                .other_link(format!("/lists/{}", id.clone().unwrap_or(String::new())).as_str()),
        };
        OutputJson {
            data_type: String::from("Task"),
            id,
            attributes: self,
            relationship: Some(relation),
        }
    }
}
