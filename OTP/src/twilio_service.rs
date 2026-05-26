use std::{collections::HashMap, env};

use axum::http::HeaderMap;
use reqwest::Client;

use crate::model::{OTPVerifyResponse, VerifyOtp};

#[derive(Clone)]
pub struct TwilioService {
    service_sid: String,
    account_sid: String,
    auth_token: String,
}

impl TwilioService {
    pub fn new() -> TwilioService {
        TwilioService {
            service_sid: env::var("SERVICE_SID").to_owned().unwrap(),
            account_sid: env::var("ACCOUNT_SID").to_owned().unwrap(),
            auth_token: env::var("AUTH_TOKEN").to_owned().unwrap(),
        }
    }

    pub async fn send_otp(&self, phone: String) -> Result<(), &'static str> {
        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = self.service_sid
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, String> = HashMap::new();

        form_body.insert("To", phone.to_string());
        form_body.insert("Channel", "sms".to_string());

        let client = Client::new();

        let res = client
            .post(url)
            .basic_auth(self.account_sid.clone(), Some(self.auth_token.clone()))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                println!("The repsonse of the send otp request is, {:?}", response);
                Ok(())
            }
            Err(err) => {
                eprintln!("Error sending OTP {}", err);
                Err("Error Sending OTP")
            }
        }
    }

    pub async fn verify_otp(&self, verify: VerifyOtp) -> Result<(), &'static str> {
        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
            serv_id = self.service_sid
        );

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();

        form_body.insert("To", &verify.phone);
        form_body.insert("Code", &verify.code);

        let client = Client::new();

        let res = client
            .post(url)
            .basic_auth(self.account_sid.clone(), Some(self.auth_token.clone()))
            .headers(headers)
            .form(&form_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                let data = response.json::<OTPVerifyResponse>().await;
                match data {
                    Ok(result) => {
                        if result.status == "approved" {
                            Ok(())
                        } else {
                            Err("Error verifying OTP")
                        }
                    }
                    Err(_) => Err("Error verifying OTP"),
                }
            }
            Err(err) => {
                eprintln!("Error Verifying OTP {}", err);
                Err("Error Verifying OTP")
            }
        }
    }
}
