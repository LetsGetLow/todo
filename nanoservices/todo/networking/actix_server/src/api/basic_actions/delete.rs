use actix_web::{HttpRequest, HttpResponse};
use shared::errors::{NanoServiceError, NanoServiceErrorStatus};
use todo_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};

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
