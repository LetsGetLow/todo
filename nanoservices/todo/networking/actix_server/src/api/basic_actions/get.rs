use actix_web::HttpResponse;

use shared::errors::NanoServiceError;
use todo_core::api::basic_actions::get::get_all as get_all_core;
pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    let response = HttpResponse::Ok().json(get_all_core().await?);
    Ok(response)
}
