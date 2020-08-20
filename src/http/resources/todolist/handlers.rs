use actix_web::{web,Responder,ResponseError,http::{header, StatusCode}, HttpResponse};
use crate::usecase::todo_usecase::{create_list,search_list,append_list,delete_list};


use crate::gateway::make_gateway;
use crate::http::rest::transform_output;
use super::io::*;
use crate::port::error::*;
use crate::http::error::*;
use crate::http::resources::utils::*;
use std::time::SystemTime;

// async fn create_list_api(input_data:web::Json<>){

// }
use crate::usecase::task_usecase::task_finder::find_one_task;
// async fn find_list_api(query:web::Path<String>){
//     let gtw = make_gateway().await;
//     let searched = search_list::execute(gtw, &query).await;
//     let exp = searched.unwrap().payload.unwrap();
    
//     let em = exp.task_store.tasks.iter().map(|task_id|{
//         find_one_task(gtw, &task_id)
//     });
// }
