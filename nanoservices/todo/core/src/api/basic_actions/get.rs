use crate::structs::{AllTodoItems, TodoItem};

use shared::errors::{NanoServiceError, NanoServiceErrorStatus};
#[cfg(feature = "json-file-storage")]
use todo_dal::json_file::get_all as get_all_handle;

/// Retrieves all todo items grouped by status.
///
/// # Returns
/// An `AllTodoItems` instance containing all stored items.
pub async fn get_all() -> Result<AllTodoItems, NanoServiceError> {
    Ok(AllTodoItems::from_hashmap(get_all_handle::<TodoItem>()?))
}

/// Retrieves a single todo item by its name.
///
/// # Arguments
/// - `name`: The name of the todo item to retrieve.
///
/// # Returns
/// The matching `TodoItem`, or a `NotFound` error if it does not exist.
pub async fn get_by_name(name: &str) -> Result<TodoItem, NanoServiceError> {
    // todo: this will be optimized later just quick and dirty for early development
    let item = get_all_handle()?.remove(name).ok_or(NanoServiceError::new(
        format!("Item with name {name} not found"),
        NanoServiceErrorStatus::NotFound,
    ))?;
    Ok(item)
}
