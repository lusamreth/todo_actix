use crate::usecase::todo_usecase::{
    append_list, create_list, delete_list, list_tasks, search_list,
};
use actix_web::{
    http::{header, StatusCode},
    web, HttpResponse, Responder, ResponseError,
};

use super::io::*;
use crate::domain::todolist::list;
use crate::gateway::make_gateway;
use crate::http::error::*;
use crate::http::resources::utils::DefaultOutput;
use crate::http::rest::{transform_output, ContentState};
use crate::port::error::*;
use std::time::SystemTime;

// async fn create_list_api(input_data:web::Json<>){

// }
pub async fn find_list_api(query: web::Path<String>) -> HttpResponse {
    let gtw = make_gateway().await;
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
    let gtw = make_gateway().await;
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

pub async fn create_list(body: web::Json<input::TodolistJson>) -> impl Responder {
    let gtw = make_gateway().await;
    let new_list = create_list::execute(gtw, body.name.clone()).await;
    dbg!("{:#?}",&body);
    println!("initiating create");
    match new_list {
        Ok(created) => {
            let insert_id = created.payload.as_ref().unwrap_or(&"".to_string()).clone();
            let location = &format!("/lists/{}", insert_id);
            let resp = transform_output(&created, location);
            match resp {
                ContentState::FULL(mut resb) => {
                    let resp = resb.json(output::CreationJson {
                        details: "Sucessfully created new list!",
                        creation_id: insert_id,
                        timestamp: 0,
                    });
                    return resp;
                }
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(err) => err.error_response(),
    }
}
