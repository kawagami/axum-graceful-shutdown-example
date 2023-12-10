use axum::http::header::HeaderMap;

pub async fn post_handler(headers: HeaderMap) -> String {
    // let custom_header_value = headers.get("Content-Type").unwrap();
    let custom_header_value = headers.get("asdgsdag").unwrap();
    let custom_header = custom_header_value.to_str().unwrap().to_string();
    custom_header
}
