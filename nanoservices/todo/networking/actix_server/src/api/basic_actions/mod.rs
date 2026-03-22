use actix_web::web::{ServiceConfig, delete, get, patch, post, scope};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

/// Registers all basic CRUD routes for todo items under `/api/v1`.
///
/// # Arguments
/// - `app`: A mutable reference to the `ServiceConfig` to register routes on.
pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("todo", get().to(get::get_all))
            .route("todo", post().to(create::create))
            .route("todo", patch().to(update::update))
            .route("todo/{name}", get().to(get::get_by_name))
            .route("todo/{name}", delete().to(delete::delete_by_name)),
    );
}
