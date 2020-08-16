use super::Gateway;
use crate::domain::resporitory_interface::Itaskresp;
use crate::domain::todolist::task::Task;
use crate::domain::todolist::task_date;
use crate::port::todo_serv::Taskport;
use crate::port::{
    error::*,
    todo_serv::{BundlePortRes, PortRes},
};
use async_trait::async_trait;

type CreationId = String;

#[async_trait(?Send)]
impl Taskport for Gateway {
    async fn find_task(&self, id: &str) -> PortRes<Task>
    where
        PortError: Exception,
    {
        let db = (self.col)().await;
        let finder = db.find_task(id).await;
        match finder {
            Ok(find_res) => {
                if let Some(doc) = find_res {
                    let task_name = doc.get("name").unwrap();
                    let desc = doc.get("description").unwrap();
                    let input_task = Task::new(task_name.to_string(), desc.to_string());
                    let existed = doc.get("created_at").unwrap().to_string();
                    let ip:Result<Task,PortException> = match input_task {
                        Ok(mut task) => {
                            task.insert_created_at(task_date::Taskdate::from_string(existed));
                            Ok(task)
                        },
                        Err(bus_err) => {
                            let bussiness_err:Result<Task,PortException> = Err(PortError::External(bus_err).domain_err());
                            return bussiness_err

                        }
                    };
                    return ip;
                } else {
                    Err(
                        PortError::External(format!("Cannot find the document with id of {}", id))
                            .operation_err(),
                    )
                }
            }
            Err(err) => {
                let db_err = PortError::convert(err);
                Err(db_err)
            }
        }
    }

    async fn create_task(&self, task_input: Task) -> PortRes<CreationId> {
        let db = (self.col)().await;
        let creator = db.insert_task(task_input).await;
        match creator {
            Ok(create_res) => {
                let id = create_res.inserted_id;
                let str_id = id.to_string();
                if str_id == String::new() {
                    Err(PortError::External("Empty string id!".to_string()).operation_err())
                } else {
                    Ok(str_id)
                }
            }
            Err(db_err) => {
                let db_err = PortError::convert(db_err);
                Err(db_err)
            }
        }
    }

    async fn delete_task(&self, id: &str) -> PortRes<bool> {
        let db = (self.col)().await;
        let deletion = db.delete_task(id).await;
        match deletion {
            Ok(delete_res) => {
                if delete_res.deleted_count > 0 {
                    Ok(true)
                } else {
                    Err(
                        PortError::External("Unable to delete the given document!".to_string())
                            .operation_err(),
                    )
                }
            }
            Err(db_err) => {
                let db_err = PortError::convert(db_err);
                Err(db_err)
            }
        }
    }

    async fn update_task(&self, id: &str, new_document: Task) -> PortRes<()> {
        let db = (self.col)().await;
        let update_opt = db.update_task(new_document, id).await;
        match update_opt {
            Ok(up_res) => {
                let match_count = up_res.matched_count;
                let confirm = up_res.modified_count;
                if match_count < 1 {
                    Err(PortError::External("Invalid Document Id!".to_string()).operation_err())
                } else if confirm < 1 {
                    Err(
                        PortError::Internal("Failed to update the given document!".to_string())
                            .operation_err(),
                    )
                } else {
                    Ok(())
                }
            }
            Err(dberr) => {
                let db_err = PortError::convert(dberr);
                Err(db_err)
            }
        }
    }
    async fn list_all(&self) -> BundlePortRes<Task> {
        let db = (self.col)().await;
        let cursor = db.find_all().await;
        let mut inner_err = None;
        match cursor {
            Ok(doc_arr) => {
                let tasks_input = doc_arr
                    .iter()
                    .map(|each_doc| {
                        let name = each_doc.get("name").unwrap().to_string();
                        let description = each_doc.get("description").unwrap().to_string();
                        let new_task_input = Task::new(name, description);

                        if let Err(ref business_error) = new_task_input {
                            inner_err = Some(PortError::External(business_error.clone()).domain_err());
                        }
                        return new_task_input.unwrap();
                    })
                    .collect::<Vec<Task>>();
                return Ok(tasks_input);
            }
            Err(bundle_err) => {
                let transformed = bundle_err
                    .iter()
                    .map(|each_err| {
                        let string_er = each_err.get_string();
                        PortError::External(string_er).operation_err()
                    })
                    .collect::<Vec<PortException>>();
                Err(transformed)
            }
        }
    }
}
// gateway is an adapter of port interface!
// Some keys insight on error handlings need!
