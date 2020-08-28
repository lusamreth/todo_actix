use crate::usecase::task_usecase::{delete_task, edit_task, make_task, task_finder, Output};
use actix_web::{
    http::{header, StatusCode},
    web, HttpResponse, Responder, ResponseError,
};
// use crate::gateway::make_gateway;
use super::io::{input::*, output::*};
use crate::gateway::make_gateway;
use crate::http::error::*;
use crate::http::resources::utils::*;
use crate::http::rest::{transform_output, ContentState};
use crate::port::error::*;
use std::time::SystemTime;

pub async fn retrieve_task_api(query: web::Path<String>) -> impl Responder {
    let gtw = make_gateway().await;
    dbg!(query.clone());
    let finder = task_finder::find_one_task(gtw, &query).await;
    match finder {
        Ok(found) => {
            let res = transform_output(&found, "/task");
            match res {
                ContentState::FULL(mut builder) => builder.json(FindingOuput {
                    id: query.to_string(),
                    task: found.payload.unwrap(),
                }),
                ContentState::EMPTY(no_ct) => no_ct,
            }
        }
        Err(err) => err.error_response(),
    }
}

pub async fn create_task_api(body: web::Json<MakeTaskJson>) -> impl Responder {
    let gtw = make_gateway().await;
    let creation = make_task::execute(gtw, body.name.clone(), body.description.clone()).await;

    match creation {
        Ok(created) => {
            let res = transform_output(&created, "/task");
            match res {
                ContentState::FULL(mut builder) => {
                    builder.status(StatusCode::CREATED);
                    builder.content_length(88888);
                    builder.json(
                        CreationOutput {
                            upsert: true,
                            message: "Sucessfully create new task!".to_string(),
                        }
                        .json_output(created.payload),
                    )
                }
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(err) => err.error_response(),
    }
}

pub async fn edit_task_api(body: web::Json<UpdateTaskJson>) -> impl Responder {
    let gtw = make_gateway().await;
    let time = body.completion.complete_time.clone();

    let target = body.target_id.clone();
    let exec = |exc| async {
        let execute = edit_task::build_executor(exc).await;
        let edition_res: Result<Output<()>, PortException> =
            execute(Box::new(gtw), body.fields.clone(), target).await;

        match edition_res {
            Ok(z) => {
                let id = body.target_id.clone();
                let located = format!("/task/{}", id);
                let res = transform_output(&z, located.as_str());
                match res {
                    ContentState::FULL(mut builder) => builder.json(z.payload),
                    ContentState::EMPTY(empty) => empty,
                }
            }
            Err(err) => err.error_response(),
        }
    };

    if body.completion.done == true {
        match time {
            Some(time) => {
                let time_res = to_usecase_time(time);
                match time_res {
                    Ok(converted_time) => {
                        let time_inj = edit_task::make_time_inj(converted_time);
                        let task_d = time_inj(("mon".to_string(), 1, 3, 2004));
                        let res: actix_web::HttpResponse = exec(Some(task_d)).await;
                        return res;
                    }
                    Err(fields) => {
                        let fe = PortException::default().invalid_input(fields);
                        return fe;
                    }
                }
            }
            None => {
                let append_field = Fielderror {
                    field: String::from("Completion"),
                    detials: "Failed Semantic in payload! Cannot complete without complete_time"
                        .to_string(),
                };
                let input_err = PortError::External(String::new())
                    .extend_input()
                    .invalid_input(vec![append_field]);
                input_err
            }
        }
    } else {
        exec(None).await
    }
}

pub async fn delete_task_api(query: web::Path<String>) -> impl Responder {
    let gtw = make_gateway().await;
    dbg!(query.clone());
    let deletion = delete_task::execute(gtw, &query).await;
    match deletion {
        Ok(output) => {
            let res_build = transform_output(&output, "");
            match res_build {
                ContentState::FULL(mut builder) => builder.json(
                    DeletionOutput {
                        acknowledgement: output.payload.unwrap(),
                        message: "Sucessfully deleted task!".to_string(),
                    }
                    .json_output(Some(query.to_string())),
                ),
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(del_err) => del_err.error_response(),
    }
}

pub async fn list_tasks_api() -> impl Responder {
    let gtw = make_gateway().await;
    let listing = task_finder::list_all_tasks(gtw).await;
    match listing {
        Ok(op) => {
            let located = "/tasks";
            let res_build = transform_output(&op, located);
            let resp = match op.payload {
                Some(vt) => FindTaskJson { payloads: vt },
                None => FindTaskJson { payloads: vec![] },
            };
            let related: Option<String> = None;
            let output = OutputJson {
                id: None,
                data_type: "Task".to_string(),
                attributes: resp,
                relationship: related, // replace relationship
            };
            match res_build {
                ContentState::FULL(mut builder) => builder.json(output),
                ContentState::EMPTY(empty) => empty,
            }
        }
        Err(op_err) => {
            let res_err = op_err
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
