use actix_web::{HttpRequest, HttpResponse};

use shared::errors::NanoServiceError;
use todo_core::api::basic_actions::get::get_all as get_all_core;
use todo_core::api::basic_actions::get::get_by_name as get_by_name_core;

/// HTTP handler that returns all todo items grouped by status.
///
/// # Returns
/// A JSON response with all todo items.
pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    let response = HttpResponse::Ok().json(get_all_core().await?);
    Ok(response)
}

/// HTTP handler that retrieves a single todo item by name.
///
/// # Arguments
/// - `req`: The incoming HTTP request containing the `name` path parameter.
///
/// # Returns
/// A JSON response with the matching todo item.
pub async fn get_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let name = req.match_info().get("name").ok_or(NanoServiceError::new(
        "Name parameter is missing".to_string(),
        shared::errors::NanoServiceErrorStatus::BadRequest,
    ))?;
    let response = HttpResponse::Ok().json(get_by_name_core(name).await?);
    Ok(response)
}
