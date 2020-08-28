use super::handlers::{
    create_task_api, delete_task_api, edit_task_api, list_tasks_api, retrieve_task_api,
};
use actix_web::{guard, web};
// scope tasks!
pub fn retrieval_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").guard(guard::Get()).to(list_tasks_api))
        .service(web::resource("/{id}").to(retrieve_task_api));
}

pub fn creation_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").guard(guard::Post()).to(create_task_api));
}

pub fn patcher_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/{id}").guard(guard::Put()).to(edit_task_api));
}

pub fn deletion_cfg(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{id}")
            .guard(guard::Delete())
            .to(delete_task_api),
    );
}
