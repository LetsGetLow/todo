use crate::structs::{AllTodoItems, TodoItem};

use shared::errors::NanoServiceError;
#[cfg(feature = "json-file-storage")]
use todo_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, NanoServiceError> {
    Ok(AllTodoItems::from_hashmap(get_all_handle::<TodoItem>()?))
}
