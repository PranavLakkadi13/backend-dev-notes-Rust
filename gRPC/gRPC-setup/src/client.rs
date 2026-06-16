use crate::helloworld::HelloRequest;
use crate::helloworld::greetings_client::GreetingsClient;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // here we are just conneting to the exitting server using the proto build file
    let mut client = GreetingsClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Alice".to_string(),
    });

    let response = client.say_hello(request).await?;

    println!("the server response is: {:?}", response.into_inner().name);

    Ok(())
}
