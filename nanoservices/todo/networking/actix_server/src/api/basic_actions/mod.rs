use actix_web::web::{ServiceConfig, get, scope};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("todo", get().to(get::get_all))
            .route("todo/{name}", get().to(get::get_by_name)),
    );
}
