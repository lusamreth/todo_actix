use super::Output;
use super::UsecaseRes;
use crate::gateway::serializer::list_task_linkage;
use crate::port::{error::PortException, io::*, todo_serv::AggregationService};

type JoinedRes<T> = UsecaseRes<Output<T>, Vec<PortException>>;

pub async fn execute(db: impl AggregationService) -> JoinedRes<Vec<JoinedOutput>> {
    let pipelines = Pipelines { pipes: Vec::new() };
    let match_pip = pipelines.push_pipe(list_task_linkage());

    let pipes = match_pip
        .pipes
        .into_iter()
        .map(|db_wrap| db_wrap.get_doc())
        .collect();
    
    let nested = db.merge_task_list(pipes).await;
    match nested {
        Ok(output) => Ok(Output {
            payload: Some(output),
        }),
        Err(agg_err) => Err(agg_err),
    }
}
