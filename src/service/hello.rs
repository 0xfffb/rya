use axum::extract::{Path, State};


use crate::AppState;

#[worker::send]
pub async fn hello_handle(state: State<AppState>, Path(message): Path<String>) -> String {
    let _ = state.kv.put("message", &message).unwrap().execute().await;
    let value = state.kv.get("message").text().await.unwrap();
    match value {
        Some(message) => { format!("kv_message_{}", message) },
        None => { format!("message_{}", message)},
    }
}
