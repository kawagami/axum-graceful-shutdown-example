use axum::Extension;

use super::SharedData;

pub async fn get_handler(Extension(shared_data): Extension<SharedData>) -> String {
    shared_data.message
}
