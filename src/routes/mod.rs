mod index;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;
mod query_params;

use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    // Router::new().merge(api::app())
    Router::new()
        .route("/", get(index::get_handler).post(index::post_handler))
        .route("/mirror_body_string", get(mirror_body_string::get_handler))
        .route("/mirror_body_json", post(mirror_body_json::post_handler))
        .route("/path_variables/:id", post(path_variables::post_handler))
        .route("/query_params", get(query_params::get_handler))
}
