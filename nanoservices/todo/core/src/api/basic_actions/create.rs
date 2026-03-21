use crate::structs::TodoItem;
use shared::errors::NanoServiceError;

#[cfg(feature = "json-file-storage")]
use todo_dal::json_file::save_one;
pub async fn create(item: TodoItem) -> Result<TodoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
