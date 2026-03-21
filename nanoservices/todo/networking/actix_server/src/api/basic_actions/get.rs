use actix_web::{HttpRequest, HttpResponse};

use shared::errors::NanoServiceError;
use todo_core::api::basic_actions::get::get_all as get_all_core;
use todo_core::api::basic_actions::get::get_by_name as get_by_name_core;

pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    let response = HttpResponse::Ok().json(get_all_core().await?);
    Ok(response)
}

pub async fn get_by_name(req: HttpRequest) -> Result<HttpResponse, NanoServiceError> {
    let name = req.match_info().get("name").ok_or(NanoServiceError::new(
        "Name parameter is missing".to_string(),
        shared::errors::NanoServiceErrorStatus::BadRequest,
    ))?;
    let response = HttpResponse::Ok().json(get_by_name_core(name).await?);
    Ok(response)
}
