use crate::structs::TodoItem;
use shared::errors::NanoServiceError;
use todo_dal::json_file::{get_all as get_all_handle, save_all};

/// Updates an existing todo item in the JSON store.
///
/// # Arguments
/// - `item`: The `TodoItem` instance containing the updated data. The `title` field is used as the
/// identifier for which item to update.
pub async fn update(item: TodoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all_handle::<TodoItem>()?;
    if !all_items.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            shared::errors::NanoServiceErrorStatus::NotFound,
        ));
    }
    all_items.insert(item.title.clone(), item);
    save_all(&all_items)?;
    Ok(())
}
