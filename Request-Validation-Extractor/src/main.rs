use axum::{Router, http::StatusCode, routing::{post}};
use tokio::net::TcpListener;

use crate::{model::Employee, validator::ValidatedPayload};

mod model;
mod validator;

async fn handler(ValidatedPayload(data): ValidatedPayload<Employee>) -> StatusCode {
    println!("{:?}", data);
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("{:?}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
