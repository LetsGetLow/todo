use actix_web::{HttpResponse, web::Json};
use shared::errors::NanoServiceError;
use todo_core::{
    api::basic_actions::{get::get_all as get_all_core, update::update as update_core},
    structs::TodoItem,
};

pub async fn update(body: Json<TodoItem>) -> Result<HttpResponse, NanoServiceError> {
    update_core(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
