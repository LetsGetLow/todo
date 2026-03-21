use std::{
    collections::HashMap,
    env,
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use serde::{Serialize, de::DeserializeOwned};
use shared::{
    errors::{NanoServiceError, NanoServiceErrorStatus},
    safe_eject,
};

/// Opens a read/write file handle to the JSON store, creating it if it does not exist.
///
/// # Arguments
/// - `path`: Optional path override. Falls back to `JSON_STORE_PATH` env var or `./tasks.json`.
///
/// # Returns
/// A `File` handle opened for reading and writing.
fn get_handle(path: Option<&str>) -> Result<File, NanoServiceError> {
    let path = match path {
        Some(p) => p,
        None => &env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string()),
    };

    let result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path);

    safe_eject!(
        result,
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON file"
    )
}

/// Opens a write-only file handle to the JSON store, truncating existing content.
///
/// # Arguments
/// - `path`: Optional path override. Falls back to `JSON_STORE_PATH` env var or `./tasks.json`.
///
/// # Returns
/// A `File` handle opened for writing.
fn get_write_handle(path: Option<&str>) -> Result<File, NanoServiceError> {
    let path = match path {
        Some(p) => p,
        None => &env::var("JSON_STORE_PATH").unwrap_or("./tasks.json".to_string()),
    };

    let result = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);

    safe_eject!(
        result,
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON file"
    )
}

/// Reads and deserializes all items from the JSON store.
///
/// # Returns
/// A `HashMap` mapping item keys to deserialized values.
pub fn get_all<T: DeserializeOwned>() -> Result<HashMap<String, T>, NanoServiceError> {
    let mut file = get_handle(None)?;
    let mut contents = String::new();
    safe_eject!(
        file.read_to_string(&mut contents),
        NanoServiceErrorStatus::Unknown,
        "Error reading JSON file to get all tasks"
    )?;
    let tasks = safe_eject!(
        serde_json::from_str(&contents),
        NanoServiceErrorStatus::Unknown,
        "Error parsing JSON file"
    )?;
    Ok(tasks)
}

/// Serializes and writes all items to the JSON store, replacing existing content.
///
/// # Arguments
/// - `tasks`: A reference to the `HashMap` of items to persist.
pub fn save_all<T: Serialize>(tasks: &HashMap<String, T>) -> Result<(), NanoServiceError> {
    let mut file = get_write_handle(None)?;
    let json = safe_eject!(
        serde_json::to_string_pretty(tasks),
        NanoServiceErrorStatus::Unknown,
        "Error serializing JSON before saving tasks"
    )?;
    safe_eject!(
        file.write_all(json.as_bytes()),
        NanoServiceErrorStatus::Unknown,
        "Error writing tasks to JSON file"
    )?;
    Ok(())
}

/// Retrieves a single item from the JSON store by its id.
///
/// # Arguments
/// - `id`: The key of the item to retrieve.
///
/// # Returns
/// The deserialized item, or an error if it does not exist.
pub fn get_one<T: DeserializeOwned + Clone>(id: &str) -> Result<T, NanoServiceError> {
    let tasks = get_all::<T>()?;
    match tasks.get(id) {
        Some(t) => Ok(t.clone()),
        None => Err(NanoServiceError::new(
            format!("Task with id {id} not found"),
            NanoServiceErrorStatus::Unknown,
        )),
    }
}

/// Inserts or updates a single item in the JSON store.
///
/// # Arguments
/// - `id`: The key under which the item is stored.
/// - `task`: A reference to the item to save.
pub fn save_one<T>(id: &str, task: &T) -> Result<(), NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone,
{
    let mut tasks = get_all::<T>().unwrap_or_else(|_| HashMap::new());
    tasks.insert(id.to_string(), task.clone());
    save_all(&tasks)
}

/// Removes a single item from the JSON store by name.
///
/// # Arguments
/// - `name`: The key of the item to delete.
///
/// # Returns
/// The deleted item, or a `NotFound` error if it does not exist.
pub fn delete_one<T>(name: &str) -> Result<T, NanoServiceError>
where
    T: Serialize + DeserializeOwned + Clone + std::fmt::Debug,
{
    let mut tasks = get_all::<T>().unwrap_or(HashMap::new());
    match tasks.remove(name) {
        Some(deleted_item) => {
            save_all(&tasks)?;
            Ok(deleted_item)
        }
        None => Err(NanoServiceError::new(
            format!("Task with title {} not found", name),
            NanoServiceErrorStatus::NotFound,
        )),
    }
}
