use axum::{ body::Body, http::response::Response, middleware::Next };
use axum::http::{Method, Request};
use worker::console_log;



pub async fn login_middleware(request: Request<Body>, next: Next) -> Response<Body>
{   
    match request.method() {
        &Method::GET => {
            console_log!("Received GET request");
            let query = request.uri().query();
            
            match query {
                Some(query) => console_log!("query: {}", query),
                None => {},
    }
        }
        &Method::POST => {
             console_log!("Received POST request")
        }
        _ => console_log!("Received other request")
    }
    console_log!("login middleware");
    let response = next.run(request).await;
    response
}
