use std::net::SocketAddr;

use axum::{
    Json, Router,
    http::{self, HeaderValue},
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

async fn html() -> impl IntoResponse {
    Html(
        r#" 
    <script>
    fetch('http://localhost:3001/data', { headers: { 'Authorization': 'application/json' } ,}).then(response => response.json()).then(data => console.log(data));
    </script>"#,
    )
}

async fn data() -> impl IntoResponse {
    Json("{message: Hello World}")
}

#[tokio::main]
async fn main() {
    let frontend = async {
        let app = axum::Router::new().route("/", get(html));
        serve(app, 3000).await
    };

    let backend = async {
        let app = Router::new().route("/data", get(data)).layer(
            CorsLayer::new()
            // expects the request from this endpoint
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                // and it expectes this header to be added in the request else will be rejected
                .allow_headers([http::header::AUTHORIZATION]),
        );
        serve(app, 3001).await;
    };

    println!("The service are running.....");

    tokio::join!(frontend, backend);
}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
