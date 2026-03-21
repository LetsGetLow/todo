use std::fmt;

use serde::{Deserialize, Serialize};
use shared::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaskStatus {
    Done,
    Pending,
}

impl TaskStatus {
    /// Converts a string to a TaskStatus enum.
    ///
    /// # Arguments
    /// * `status` - A reference to a string that is to be converted to a TaskStatus enum
    ///
    /// # Returns
    /// the constructe TaskStatus enum if the string is valid, otherwise an error message
    pub fn from_string(status: &String) -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::Done),
            "PENDING" => Ok(TaskStatus::Pending),
            _ => Err(NanoServiceError::new(
                "Invalid status".to_string(),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Done => write!(f, "DONE"),
            Self::Pending => write!(f, "PENDING"),
        }
    }
}
