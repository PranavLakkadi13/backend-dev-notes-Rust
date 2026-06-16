use crate::helloworld::HelloRequest;
use crate::helloworld::greetings_client::GreetingsClient;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;

    let token: MetadataValue<_> = "Bearer abcdefg".parse()?;

    // here we are just conneting to the exitting server using the proto build file
    // let mut client = GreetingsClient::connect("http://[::1]:50051").await?;

    // here we are buidling the teh greeter and add the bearer token to it metadata
    let mut client = GreetingsClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    } );

    let request = tonic::Request::new(HelloRequest {
        name: "Alice".to_string(),
    });

    let response = client.say_hello(request).await?;

    println!("the server response is: {:?}", response.into_inner().name);

    Ok(())
}
