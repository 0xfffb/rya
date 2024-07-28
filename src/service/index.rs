use axum::response::{ IntoResponse, Response};

use crate::model::index::IndexTemplate;


#[worker::send]
pub async fn index_handle() -> Response {
    IndexTemplate::new("index".to_string(), "this is index page.".to_string()).into_response()
}