use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TodolistJson {
    pub name: String,
    pub task_lists: Vec<String>,
}
