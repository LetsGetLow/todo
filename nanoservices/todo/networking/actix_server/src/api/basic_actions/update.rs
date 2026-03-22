use actix_web::{HttpResponse, web::Json};
use shared::errors::NanoServiceError;
use todo_core::{
    api::basic_actions::{get::get_all as get_all_core, update::update as update_core},
    structs::TodoItem,
};

/// HTTP handler that updates an existing todo item based on the provided JSON body.
///
/// # Arguments
/// * `body`: A JSON payload containing the updated `TodoItem`. The `title` field is used as the
/// identifier for which item to update.
///
/// # Returns
/// A JSON response with all todo items after the update, or an error if the update fails
pub async fn update(body: Json<TodoItem>) -> Result<HttpResponse, NanoServiceError> {
    update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
