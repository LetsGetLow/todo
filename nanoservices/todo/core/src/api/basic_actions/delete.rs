use shared::errors::NanoServiceError;
use todo_dal::json_file::delete_one;

use crate::structs::TodoItem;

/// Deletes a todo item.
///
/// # Arguments
/// - `id` - a string slice that specifies the id of the item to delete.
///
/// # Returns
/// The deleted item.
pub async fn delete(name: &str) -> Result<TodoItem, NanoServiceError> {
    delete_one::<TodoItem>(name)
}
