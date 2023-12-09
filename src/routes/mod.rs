mod index;
mod mirror_body_string;

use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    // Router::new().merge(api::app())
    Router::new()
        .route("/", get(index::get_handler).post(index::post_handler))
        .route("/mirror_body_string", get(mirror_body_string::get_handler))
}