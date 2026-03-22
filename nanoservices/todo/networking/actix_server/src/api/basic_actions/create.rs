use actix_web::{HttpResponse, web::Json};
use shared::errors::NanoServiceError;
use todo_core::{
    api::basic_actions::{create::create as create_core, get::get_all as get_all_core},
    structs::TodoItem,
};

/// HTTP handler that creates a new todo item and returns all current items.
///
/// # Arguments
/// - `body`: JSON body containing the `TodoItem` to create.
///
/// # Returns
/// A JSON response with all todo items after the creation.
pub async fn create(body: Json<TodoItem>) -> Result<HttpResponse, NanoServiceError> {
    create_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
