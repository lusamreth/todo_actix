use serde::{Deserialize, Serialize};
use crate::http::resources::task::io;
use io::input::MakeTaskJson;
#[derive(Debug, Deserialize, Serialize)]
pub struct TodolistJson {
    pub name: String,
    pub task_lists: Vec<MakeTaskJson>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeletionJson{
    pub list_id:String,
    pub targets:Vec<String>
}