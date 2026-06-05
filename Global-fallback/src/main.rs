use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;

async fn health_handler() -> Html<&'static str> {
    Html("<h1>up and running....</h1>")
}

async fn fallback_handler() -> Html<&'static str> {
    Html("<h1>This route doesnt exist</h1>")
}

#[warn(dead_code)]
async fn fallback_handler2() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "NO Route Found")
}

#[tokio::main]
async fn main() {
    // Usually when a service is running it has the certain endpoints
    // eg /health
    // fallback is basically a method that is triggered when a unknown endpoint is hit eg /hi
    // this allows graceful handling of unkown endpoints instead of throwing doesnt exist error and 404
    let app = axum::Router::new()
        .route("/health", get(health_handler))
        // this how u have a global fallback
        .fallback(fallback_handler);
        // u should not have fallbacks like this
        // in this case the lower one will always be triggered refer README.md
        // .fallback(fallback_handler2);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
