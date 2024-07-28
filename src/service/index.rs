use axum::response::{ IntoResponse, Response};

use crate::model::index::IndexTemplate;


#[worker::send]
pub async fn index_handle() -> Response {
    IndexTemplate::new("index", "this is index page.").into_response()
}