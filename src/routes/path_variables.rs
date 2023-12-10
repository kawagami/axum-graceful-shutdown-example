use axum::extract::Path;

pub async fn post_handler(Path(id): Path<u32>) -> String {
    id.to_string()
}
