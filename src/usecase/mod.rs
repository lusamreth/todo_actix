pub mod task_usecase;
pub mod todo_usecase;
#[allow(dead_code)]
pub(crate) type UsecaseRes<T, E> = Result<T, E>;
#[allow(dead_code)]
pub struct Output<T> {
    payload: Option<T>,
}
