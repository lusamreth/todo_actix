use super::*;
use crate::domain::todolist::task::Task;
use crate::port::{error::PortException, todo_serv::*};

type MultipleFinderRes<T> = UsecaseRes<Output<T>, MultipleExceptions>;
type OneFinderRes<T> = UsecaseRes<Output<T>, PortException>;

pub async fn list_all_tasks(repo: impl Taskport) -> MultipleFinderRes<Vec<Task>> {
    let retrieval = repo.list_all().await;
    // still retrive the same functionality
    match retrieval {
        Ok(task_arr) => Ok(Output {
            payload: Some(task_arr),
        }),
        Err(bundle_err) => Err(bundle_err),
    }
}

pub async fn find_one_task(repo: impl Taskport, id: &str) -> OneFinderRes<Task> {
    let finder = repo.find_task(id).await;
    match finder {
        Ok(found) => Ok(Output {
            payload: Some(found),
        }),
        Err(err) => Err(err),
    }
}
use futures::{Stream,stream};

type ManyFinderRes<T> = UsecaseRes<Output<T>,MultipleExceptions>;

