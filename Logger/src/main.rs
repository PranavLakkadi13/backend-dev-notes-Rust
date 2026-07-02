use std::env;

use log::{debug, error, info, warn};

fn main() {
    // so like the error logs form the logger has a selective range that we can choose 
    // error!() will only print the error!
    // warn!() will print both error! and warn! macros
    // info!() will print all error! warn! and info!
    // debug!() will print all the types of the macros mentioned below
    unsafe {
        env::set_var("RUST_LOG", "debug");
        // env::set_var("RUST_LOG", "error");
        // env::set_var("RUST_LOG", "warn");
        // env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let username = "admn";
    let password = "admi";

    debug!("checking credentials....");

    if username.is_empty() || password.is_empty() {
        error!("Credentials are empty");
        return;
    }

    if username == "admin" && password == "admin" {
        info!("Its admin user....");
    } else {
        warn!("The username or password is invalid");
    }
}
