use askama_axum::IntoResponse;
use axum::{http::Uri, response::Response};

use crate::model::not_found::NotFound;


pub async fn not_found_handle(uri: Uri) -> Response {
    NotFound::new(format!("not found: {}", uri.path().to_string())).into_response()
}