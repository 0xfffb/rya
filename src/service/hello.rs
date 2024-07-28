use askama_axum::IntoResponse;
use axum::extract::{Path, State};
use axum::response::Response;

use crate::model::hello::HelloTemplate;
use crate::AppState;

#[worker::send]
pub async fn hello_handle(state: State<AppState>, Path(message): Path<String>) -> Response {
    let _ = state.kv.put("hello", &message).unwrap().execute().await;
    let value = state.kv.get("hello").text().await.unwrap();
    match value {
        Some(message) => { 
            HelloTemplate::new(message).into_response()
        },
        None => { 
            HelloTemplate::new("getting kv is error".to_string()).into_response()
        },
    }
}
