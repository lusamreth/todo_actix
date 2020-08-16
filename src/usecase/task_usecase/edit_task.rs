use super::*;
use crate::domain::todolist::{task::Task, task_date::*};
use crate::port::error::*;
use crate::port::todo_serv::Taskport;
use chrono::Weekday;
type EditRes<T> = UsecaseRes<Output<T>, PortException>;
use serde::Serialize;
use std::str::FromStr;
#[derive(Serialize)]
struct UpdateTaskField {
    name: String,
    desc: String,
    pub completion: bool,
}

impl UpdateTaskField {
    pub fn new() -> Self {
        return UpdateTaskField {
            name: String::new(),
            desc: String::new(),
            completion: false,
        };
    }
}

pub fn make_time_inj(t: (i64, i64, i64)) -> TimeInjector {
    let oz = Taskdate::make_time(t.0, t.1, t.2);
    let inject_date = move |tp: Timeparam| {
        let weekday = Weekday::from_str(tp.0.as_str()).expect("Wrong weekday format!");
        Taskdate::new(weekday, tp.1 as u32, tp.2 as u32, tp.3, &oz)
    };
    return Box::new(inject_date);
}
#[allow(dead_code)]
type ClosureFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<Output<()>, PortException>>>>;

#[allow(dead_code)]
type ExecFun = Box<dyn FnOnce(Box<dyn Taskport>, UpdateTaskField, String) -> ClosureFuture>;
#[allow(dead_code)]
type Timeparam = (String, i64, i64, i32);
#[allow(dead_code)]
type TimeInjector = Box<dyn Fn(Timeparam) -> Taskdate>;

#[allow(dead_code)]
async fn proxy(
    repo: Box<dyn Taskport>,
    new_fields: Task,
    id: String,
    invoke_err: Option<PortException>,
) -> EditRes<()> {
    let update = repo.update_task(&id, new_fields).await;
    if let Some(invoke_err) = invoke_err {
        let throwed: Result<Output<()>, PortException> = Err(invoke_err);
        return throwed
    }
    match update {
        Ok(_) => Ok(Output { payload: None }),
        Err(update_err) => Err(update_err),
    }
}

async fn build_executor(comp_time: Option<Taskdate>) -> ExecFun {
    let closure =
        |repo: Box<dyn Taskport>, new_fields: UpdateTaskField, id: String| -> ClosureFuture {
            let input_name = new_fields.name.to_string();
            let input_desc = new_fields.desc.to_string();
            let mut catch_err: Option<PortException> = None;
            let mut new_task = Task::new(input_name, input_desc);
            let mut buffer= Vec::with_capacity(1);

            match new_task {
                Ok(mut new_task) => { 
                    if let Some(date) = comp_time {
                        new_task.finish(date)
                    } else {
                        if new_task.completion == true {
                            let throwable = PortError::External(String::from(
                                "Missing completion time eventhough task is complete!",
                            ))
                            .assert_type("InvalidField".to_string(), "Missing-field".to_string());
                            catch_err = Some(throwable)
                        } else {
                            catch_err = None
                        }
                    }
                    new_task.insert_modified_at(Taskdate::new_local());
                    buffer.push(new_task)
                },
                Err(domain_err) => {
                    catch_err = Some(PortError::External(domain_err).domain_err());
                }
            }
            if let None = buffer.get(1){
                catch_err = Some(PortError::External(String::from("Empty buffer!")).extend_input())
            };
            let oem = Box::pin(proxy(repo, buffer.get(1).unwrap().to_owned(), id.to_string(), catch_err));
            return oem;
        };

    let hmm = Box::new(closure);
    return hmm;
    // return closure;
}
