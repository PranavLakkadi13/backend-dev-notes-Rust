use axum::{Extension, Json, Router, routing::post};
use reqwest::StatusCode;
use tokio::net::TcpListener;

use crate::{
    model::{SignInRequest, VerifyOtp},
    twilio_service::TwilioService,
};

mod model;
mod twilio_service;

pub async fn sign_in(
    service: Extension<TwilioService>,
    Json(data): Json<SignInRequest>,
) -> Result<Json<String>, StatusCode> {
    let number = verify_data(data);

    match service.send_otp(number).await {
        Ok(_) => Ok(Json("message: Please enter your otp".parse().unwrap())),
        Err(ex) => {
            eprintln!("the error is {:?}", ex);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

async fn verify(
    service: Extension<TwilioService>,
    Json(data): Json<VerifyOtp>,
) -> Result<Json<String>, StatusCode> {
    match service.verify_otp(data).await {
        Ok(_) => Ok(Json("message: Successsss!!!!!".parse().unwrap())),
        Err(ex) => {
            eprintln!("the error is {:?}", ex);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub fn verify_data(_sign_in_request: SignInRequest) -> String {
    // U should do this below things generallly
    // simulate verification from db
    // simulate get phone number against email
    String::from("+919000540262") // E.164 format: +91 (India) + 10-digit number
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let service = TwilioService::new();

    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/verify", post(verify))
        .layer(Extension(service));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("The service is up and running....");

    axum::serve(listener, app).await.unwrap();
}
