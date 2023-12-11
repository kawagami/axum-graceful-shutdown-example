mod always_errors;
mod get_json;
mod index;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod returns_201;
mod set_middleware_custom_header;
mod shared_data;

use axum::{
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct SharedData {
    message: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "shared data in middleware".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(index::get_handler).post(index::post_handler))
        .route("/mirror_body_string", get(mirror_body_string::get_handler))
        .route("/mirror_body_json", post(mirror_body_json::post_handler))
        .route("/path_variables/:id", post(path_variables::post_handler))
        .route("/query_params", get(query_params::get_handler))
        .route("/mirror_user_agent", post(mirror_user_agent::post_handler))
        .route("/shared_data", get(shared_data::get_handler))
        .layer(Extension(shared_data))
        .layer(cors)
        .route("/always_errors", get(always_errors::always_errors))
        .route("/returns_201", post(returns_201::returns_201))
        .route("/get_json", get(get_json::get_json))
}
