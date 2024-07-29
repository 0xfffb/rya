
use askama_axum::IntoResponse;
use axum::{http::Uri, response::Response};
use crate::view::fallback::FallbackTemplate;


pub async fn fallback_handle(uri: Uri) -> Response {
    FallbackTemplate::new(format!("not found: {}", uri.path().to_string())).into_response()
}