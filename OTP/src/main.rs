use std::env;

use axum::{Extension, Json, Router, extract::State, routing::post};
use reqwest::StatusCode;
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::{
    model::{DBSchema, SignInRequest, VerifyOtp},
    twilio_service::TwilioService,
};

mod model;
mod twilio_service;

pub async fn sign_in(
    service: Extension<TwilioService>,
    State(db_state): State<PgPool>,
    Json(data): Json<SignInRequest>,
) -> Result<Json<String>, StatusCode> {
    let number = verify_data(State(db_state), data).await;

    match number {
        Ok(num) => match service.send_otp(num).await {
            Ok(_) => Ok(Json("message: Please enter your otp".parse().unwrap())),
            Err(ex) => {
                eprintln!("the error is {:?}", ex);
                Err(StatusCode::UNAUTHORIZED)
            }
        },
        Err(err) => {
            eprintln!("THe error is {:?} ", &err);
            Err(err)
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

pub async fn verify_data(
    State(state): State<PgPool>,
    sign_in_request: SignInRequest,
) -> Result<String, StatusCode> {
    // U should do this below things generallly
    // simulate verification from db
    let db_val = sqlx::query_as!(
        DBSchema,
        "Select * from users where email = $1",
        sign_in_request.email
    )
    .fetch_one(&state)
    .await
    .expect("User Not Exist");

    if &db_val.password == &sign_in_request.password {
        Ok(db_val.phone)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let service = TwilioService::new();

    let db_url = env::var("DATABASE_URL").expect("ERROR WITH DB URL");

    let pool = PgPool::connect(&db_url).await.expect("Connection failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed...");

    let app = Router::new()
        .route("/signin", post(sign_in))
        .route("/verify", post(verify))
        .layer(Extension(service))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("The service is up and running....");

    axum::serve(listener, app).await.unwrap();
}
