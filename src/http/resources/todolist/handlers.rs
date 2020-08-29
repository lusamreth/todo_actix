use crate::usecase::todo_usecase::{
    append_list, create_list, delete_list, list_tasks, search_list,remove_task
};
use actix_web::{
    http::{header, StatusCode},
    web, HttpResponse, Responder, ResponseError,
};

use super::io::*;
use crate::domain::todolist::list;
use crate::gateway::{list_gateway,task_gateway};
use crate::http::error::*;
use crate::http::resources::utils::DefaultOutput;
use crate::http::rest::{transform_output, ContentState};
use crate::port::error::*;
use std::time::SystemTime;

// async fn create_list_api(input_data:web::Json<>){

// }
pub async fn find_list_api(query: web::Path<String>) -> HttpResponse {
    let gtw = list_gateway().await;
    let searched = search_list::execute(gtw, &query).await;

    match searched {
        Ok(res) => {
            let output = transform_output(&res, &format!("/todolist/{}", &query.to_string()));
            match output {
                ContentState::FULL(mut resp) => {
                    let output = output::FinderJson {
                        todolist: res.payload.unwrap(),
                    }
                    .json_output(Some(query.to_string()));
                    resp.json(output)
                }
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(err) => err.error_response(),
    }
}

pub async fn all_lists_api() -> HttpResponse {
    let gtw = list_gateway().await;
    let list = list_tasks::execute(gtw).await;
    match list {
        Ok(res) => {
            let resp = transform_output(&res, "");
            match resp {
                ContentState::FULL(mut fpb) => {
                    let output = output::FinderJson {
                        todolist: res.payload.unwrap(),
                    }
                    .json_output(None);
                    fpb.json(output)
                }
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(b_err) => {
            let res_err = b_err
                .into_iter()
                .map(|e| {
                    let hmm = e.into();
                    return hmm;
                })
                .collect::<Vec<ErrorResponse<String>>>();
            let backed_err = BundleError {
                message: String::from("Multiple errors occured during finding!"),
                errors: res_err,
            };
            let mut http_res = HttpResponse::build(StatusCode::MULTI_STATUS);
            http_res.set(header::Date(SystemTime::now().into()));

            http_res.json(backed_err)
        }
    }
}
use crate::usecase::{task_usecase::{make_task,delete_task},Output};
use futures::{future::join_all, executor::block_on};



pub async fn create_list(body: web::Json<input::TodolistJson>) -> impl Responder {
    let gtw = list_gateway().await;
    let new_list = create_list::execute(gtw, body.name.clone()).await;
    dbg!("{:#?}",&body);
    println!("initiating create");
    let len = body.task_lists.len();
    // let mut bulk_writes:Vec<Result<String,PortException>> = Vec::new();

    match new_list {
        Ok(created) => {
            let insert_id = created.payload.as_ref().unwrap_or(&"".to_string()).clone();
            let location = &format!("/lists/{}", insert_id);
            let resp = transform_output(&created, location);
            match resp {
                ContentState::FULL(mut resb) => {
                    let mut insert_ids = Vec::new();

                    let mut ecc_col = Vec::new();
    
                    if  len > 0 {
                        let mut buffers = Vec::new();
                        body.task_lists.iter().for_each(|task_input| {
                            let hmm = block_on(task_gateway());
                            let handler = make_task::execute(hmm, task_input.name.clone(), task_input.description.clone());
                            // let hm = block_on(mk);
                            buffers.push(handler);
                        });
                        let res:Vec<Result<Output<String>,PortException>> = join_all(buffers).await;
                        res.into_iter().for_each(|handler|{
                            
                            if let Ok(id) = handler{
                                let insert_id = id.payload.unwrap_or(String::new());
                                insert_ids.push(insert_id);
                            }else if let Err(ie) = handler{
                                let resp_err:ErrorResponse<String> = ie.into();
                                ecc_col.push(resp_err)
                            }
                        });
                        
                    }
                    let appendance = append_list::execute(list_gateway().await, insert_id.clone(), insert_ids.clone()).await;

                    let has_err = ecc_col.len() > 0;
                    let bundle_err = BundleError{
                        message: "Multiple errors occur during appending task!".to_string(),
                        errors: ecc_col,
                    };
                    let resp = resb.json(output::CreationJson {
                        details: "Sucessfully created new list!",
                        appended_tasks:insert_ids,
                        timestamp: 0,
                    });
                    if has_err {
                        resb.status(StatusCode::BAD_REQUEST).json(bundle_err)
                    }else if let Err(ap_err) = appendance{
                        ap_err.error_response()
                    }
                    else{
                        return resp
                    }
                },
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(err) => err.error_response(),
    }
}

use crate::domain::todolist::task_date::Taskdate;

pub async fn list_deletion(query:web::Path<String>) -> impl Responder{
    let gtw = list_gateway().await;
    let deletion = delete_list::execute(gtw, query.to_string().clone()).await;
    let finder = search_list::execute(list_gateway().await, &query).await;
    let mut buffer_err = Vec::with_capacity(3);
    let mut tasks_id = Vec::new();

    match finder {
        Ok(found_item) => {
            if let Some(item) = found_item.payload{
                item.task_store.tasks.into_iter().for_each(|itm| tasks_id.push(itm));
            }
        }
        Err(err) => buffer_err.push(err)
    }


    match deletion{
        Ok(del_output) => {
            let mut task_del_count = 0;
            let aggregate_tasks = delete_task::multiple_execute(task_gateway().await, tasks_id).await;
            match aggregate_tasks{
                Ok(itm) => {
                    let num = itm.payload.unwrap_or(0);
                    task_del_count += num;
                }
                Err(tk_err) => {
                    buffer_err.push(tk_err.into())
                }
            }
            if buffer_err.len() > 0 {
                let errors = buffer_err.into_iter().map(|itm| itm.into()).collect::<Vec<ErrorResponse<String>>>();
                let bundle_err = BundleError{
                    message:String::from("Multiple errors during deleting!"),
                    errors
                };
                return HttpResponse::build(StatusCode::BAD_REQUEST).json(bundle_err);
            }else{
                match transform_output(&del_output, ""){
                    ContentState::FULL(mut resb) => {
                        resb.json(output::DeletionJson{ acknowledegement: true, deletion_type: output::DelType::HARD, deleted_date: Taskdate::new_local().to_string(), task_count: task_del_count}.json_output(Some(query.to_string())))
                    },
                    ContentState::EMPTY(null) => {
                        null
                    }
                }
            }
            
        }
        Err(del_err) => del_err.error_response()
    }
    
}


pub async fn remove_task(body: web::Json<input::DeletionJson>) -> impl Responder{
    let gtw = list_gateway().await;
    let rmt = remove_task::execute(gtw, body.list_id.clone(), body.targets.clone()).await;

    match rmt{
        Ok(_) => {
            let aggregate_tasks = delete_task::multiple_execute(task_gateway().await, body.targets.clone()).await;
            match aggregate_tasks{
                Ok(done) => {
                    let num = done.payload.unwrap_or(0);
                    let transformed = transform_output(&done, "/tasks");
                    match transformed{
                        ContentState::FULL(mut resb) => {
                            resb.json(output::DeletionJson{
                                acknowledegement:true,
                                deletion_type:output::DelType::SOFT,
                                deleted_date:Taskdate::new_local().to_string(),
                                task_count:done.payload.unwrap_or(0)
                            })
                        }
                        ContentState::EMPTY(null) => null
                    }
                }
                Err(err) => err.error_response()
            }

        }
        Err(err) => err.error_response()

    }
}