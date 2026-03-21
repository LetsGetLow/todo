use actix_web::{HttpRequest, HttpResponse};
use shared::errors::{NanoServiceError, NanoServiceErrorStatus};
use todo_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};

/// HTTP handler that deletes a todo item by name and returns all remaining items.
///
/// # Arguments
/// - `req`: The incoming HTTP request containing the `name` path parameter.
///
/// # Returns
/// A JSON response with all remaining todo items after deletion.
pub async fn delete_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    match req.match_info().get("name") {
        Some(name) => delete_core(name).await?,
        None => {
            return Err(NanoServiceError::new(
                "Name parameter is missing".to_string(),
                NanoServiceErrorStatus::Unknown,
            ));
        }
    };
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
