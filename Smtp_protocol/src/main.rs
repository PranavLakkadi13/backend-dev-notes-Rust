use std::env;

use lettre::{Message, SmtpTransport, Transport, transport::smtp::authentication::Credentials};

fn main() {
    dotenvy::dotenv().ok();

    let email = Message::builder()
        .from("cbac@gmail.com".parse().unwrap())
        .to("abcd@gmail.com".parse().unwrap())
        .subject("TESTING EMAIL IN RUST")
        .body(String::from("This is a Test email"))
        .unwrap();

    let cred = Credentials::new(
        "cbac@gmail.com".to_string(),
        env::var("APP_PASS").to_owned().unwrap().to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(cred)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfullyyyy"),
        Err(err) => eprintln!("email send failed... {:?}", err),
    }
}
