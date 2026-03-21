use crate::enums::TaskStatus;
use crate::structs::TodoItem;
use shared::errors::NanoServiceError;

#[cfg(feature = "json-file-storage")]
use todo_dal::json_file::save_one;
pub fn create(title: &str, status: TaskStatus) -> Result<TodoItem, NanoServiceError> {
    let item = TodoItem {
        title: title.to_string(),
        status,
    };
    let _ = save_one(&title.to_string(), &item)?;
    Ok(item)
}
