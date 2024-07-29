mod middleware;
mod service;
mod model;
mod view;


use axum::{routing::get, Router};
use kv::KvStore;
use tower_service::Service;
use worker::*;
use middleware::login_middleware;
use service::{fallback::fallback_handle, hello::hello_handle, index::index_handle, users::users_handle};


#[derive(Clone)]
struct AppState {
    pub _env: Env,
    pub kv: KvStore,
    pub d1: D1Database
}

impl AppState {
    fn new(env: Env) -> Self {
        let kv = match env.kv("rya-worker") {
            Ok(kv) => kv,
            Err(err) => panic!("{:?}", err),
        };
        let d1 = env.d1("rya-worker").unwrap();
        Self {
            _env: env,
            kv,
            d1
        }
    }
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handle))
        .route("/hello/:message", get(hello_handle))
        .route("/users/", get(users_handle))
        .layer(axum::middleware::from_fn(login_middleware))
        .fallback(fallback_handle)
        .with_state(state)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    let status = AppState::new(env);
    Ok(router(status).call(req).await?)
}
