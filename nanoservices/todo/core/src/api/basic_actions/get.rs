use crate::structs::{AllTodoItems, TodoItem};

use shared::errors::{NanoServiceError, NanoServiceErrorStatus};
#[cfg(feature = "json-file-storage")]
use todo_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, NanoServiceError> {
    Ok(AllTodoItems::from_hashmap(get_all_handle::<TodoItem>()?))
}

pub async fn get_by_name(name: &str) -> Result<TodoItem, NanoServiceError> {
    // todo: this will be optimized later just quick and dirty for early development
    let item = get_all_handle()?.remove(name).ok_or(NanoServiceError::new(
        format!("Item with name {name} not found"),
        NanoServiceErrorStatus::NotFound,
    ))?;
    Ok(item)
}
