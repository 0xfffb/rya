use askama_axum::IntoResponse;
use axum::extract::State;
use axum::response::Response;

use crate::model::user::User;
use crate::view::{index::IndexTemplate, users::UsersTemplate};
use crate::AppState;

#[worker::send]
pub async fn users_handle(state: State<AppState>) -> Response {
    let users = state.d1.prepare("select * from users").all::<User>().await.unwrap();
    IndexTemplate::new("index".to_string(), "this is index page.".to_string()).into_response()
}