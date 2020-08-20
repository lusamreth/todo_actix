use crate::gateway::serializer::{match_pipeline,Matchstage, list_task_linkage};
use crate::port::{io::*,todo_serv::AggregationService};

// async fn execute(db: impl AggregationService){
//     let pipelines = Pipelines{ pipes: Vec::new()};
//     let match_pip = pipelines.push_pipe(DatabaseDoc::create(list_task_linkage()));
//     let pipes = match_pip.pipes.iter().map(|db_wrap|{
//         db_wrap.get_doc()
//     }).collect();
//     db.merge_task_list(pipes);

// }
