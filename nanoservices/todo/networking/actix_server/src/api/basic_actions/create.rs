use actix_web::{HttpResponse, web::Json};
use shared::errors::NanoServiceError;
use todo_core::{
    api::basic_actions::{create::create as create_core, get::get_all as get_all_core},
    structs::TodoItem,
};

pub async fn create(body: Json<TodoItem>) -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
