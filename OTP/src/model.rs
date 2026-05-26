use serde::Deserialize;

// User will request to sign-in
#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

// user sharing the otp with us
#[derive(Deserialize)]
pub struct VerifyOtp {
    pub phone: String,
    pub code: String,
}

// The response twilio will be sending us
#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct DBSchema {
    pub id: String,
    pub email: String,
    pub password: String,
    pub phone: String,
}
