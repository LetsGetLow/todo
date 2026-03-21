#[cfg(feature = "actix")]
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

use serde::{Deserialize, Serialize};
use std::fmt::{self};
use thiserror::Error;

/// The status of the error.
///
/// # Notes
/// This is used to determine the HTTP status code to return.
#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum NanoServiceErrorStatus {
    #[error("Requested resource was not found")]
    NotFound,
    #[error("You are forbidden to access requested resource")]
    Forbidden,
    #[error("Unknown Internal Error")]
    Unknown,
    #[error("Bad Request")]
    BadRequest,
    #[error("Conflict")]
    Conflict,
    #[error("Unauthorized")]
    Unauthorized,
}

/// The error struct that is passed between layers and servers.
///
/// # Fields
/// * `message` - The error message.
/// * `status` - The status of the error.
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus,
}

impl NanoServiceError {
    pub fn new(message: String, status: NanoServiceErrorStatus) -> Self {
        Self { message, status }
    }
}

impl fmt::Display for NanoServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Implementing the ResponseError trait for actix_web servers.
#[cfg(feature = "actix")]
impl ResponseError for NanoServiceError {
    /// The status code for the actix web error response.
    ///
    /// # Returns
    /// The status code for the error.
    fn status_code(&self) -> StatusCode {
        match self.status {
            NanoServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
            NanoServiceErrorStatus::Forbidden => StatusCode::FORBIDDEN,
            NanoServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            NanoServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            NanoServiceErrorStatus::Conflict => StatusCode::CONFLICT,
            NanoServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }

    /// Constructs an HTTP response for the error.
    ///
    /// # Returns
    /// An HTTP response with the appropriate status code and error message.
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        HttpResponse::build(status_code).json(self.message.clone())
    }
}

/// Maps Result error to NanoServiceError to have less boilerplate code when mapping errors to
/// NanoServiceError in the codebase.
///
/// # Arguments
/// * `$e` - result that should be mapped to NanoServiceError if it is an error
/// * `$err_status - `NanoServiceErrorStatus` of the error
/// * `$message_context` - optional, error context message i.e. 'Invalid request occured'
#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $err_status:expr) => {
        $e.map_err(|x| NanoServiceError::new(x.to_string(), $err_status))
    };
    ($e:expr, $err_status:expr, $message_context:expr) => {
        $e.map_err(|x| {
            NanoServiceError::new(
                format!("{}:{}", $message_context, x.to_string()),
                $err_status,
            )
        })
    };
}
