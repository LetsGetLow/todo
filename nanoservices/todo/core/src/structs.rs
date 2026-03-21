use crate::enums::TaskStatus;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// A struct representing a todo item.
///
/// # Fields
/// - `title`: The title of the todo item.
/// - `status`: The status of the todo item.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub status: TaskStatus,
}

impl fmt::Display for TodoItem {
    /// Formats the `ToDoItem` struct.
    ///
    /// # Arguments
    /// - `f`: A mutable reference to a `fmt::Formatter`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.status {
            TaskStatus::Pending => write!(f, "Pending: {}", self.title),
            TaskStatus::Done => write!(f, "Done: {}", self.title),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllTodoItems {
    pub pending: Vec<TodoItem>,
    pub done: Vec<TodoItem>,
}

impl AllTodoItems {
    pub fn from_hashmap(all_items: HashMap<String, TodoItem>) -> AllTodoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();
        for (_, item) in all_items {
            match item.status {
                TaskStatus::Done => done.push(item),
                TaskStatus::Pending => pending.push(item),
            }
        }
        AllTodoItems { pending, done }
    }
}
