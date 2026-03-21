pub mod basic_actions;

use actix_web::web::ServiceConfig;

/// Registers all view factories with the Actix app.
///
/// # Arguments
/// - `app`: A mutable reference to the `ServiceConfig` to register routes on.
pub fn views_factory(app: &mut ServiceConfig) {
    basic_actions::basic_actions_factory(app);
}
