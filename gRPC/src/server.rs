use std::net::SocketAddr;

use tonic::{Request, Response, transport::Server};

use crate::helloworld::{
    HelloRequest, HelloResponse,
    greetings_server::{Greetings, GreetingsServer},
};

// using tonic to get the proto build files since build.rs when run generates the build file 
pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
// the Greetings is the Service form the proto file
impl Greetings for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<tonic::Response<HelloResponse>, tonic::Status> {
        let reply = HelloResponse {
            name: format!("Hello {} from server....", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: Option<SocketAddr> = "[::1]:50051".parse().ok();
    let greeter_service = MyGreeter::default();

    println!("running....");

    Server::builder()
        .add_service(GreetingsServer::new(greeter_service))
        .serve(address.unwrap())
        .await?;

    Ok(())
}
