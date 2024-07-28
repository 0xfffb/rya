mod middleware;
mod service;
mod model;


use axum::{routing::get, Router};
use kv::KvStore;
use tower_service::Service;
use worker::*;
use middleware::login_middleware;
use service::{hello::hello_handle, index::index_handle, not_found::not_found_handle};


#[derive(Clone)]
struct AppState {
    pub _env: Env,
    pub kv: KvStore
}

impl AppState {
    fn new(env: Env) -> Self {
        let kv = match env.kv("rya-worker") {
            Ok(kv) => kv,
            Err(err) => panic!("{:?}", err),
        };
        Self {
            _env: env,
            kv
        }
    }
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handle))
        .route("/hello/:message", get(hello_handle))
        .layer(axum::middleware::from_fn(login_middleware))
        .fallback(not_found_handle)
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
