use axum::{Json, Router, routing::get};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;

async fn handler() -> Json<String> {
    dotenv().ok();
    let instance_name = env::var("INSTANCE_NAME").expect("ENV value couldn't be read");

    Json::from(instance_name)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listeningggggg........ ");
    axum::serve(listener, app).await.unwrap();
}
