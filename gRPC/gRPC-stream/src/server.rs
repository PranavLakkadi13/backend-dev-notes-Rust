use std::{error::Error, iter::repeat, net::ToSocketAddrs, pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt, wrappers::ReceiverStream};
use tonic::{Request, Response, Status, transport::Server};

use crate::h2::server::pb::{StreamRequest, StreamResponse};

pub mod pb {
    tonic::include_proto!("stream");
}

type StreamResult<T> = Result<Response<T>, Status>;
// the send is to send the response
type ResponseStream = Pin<Box<dyn Stream<Item = Result<pb::StreamResponse, Status>> + Send>>;

#[derive(Debug)]
pub struct EchoServer {}

#[tonic::async_trait]
impl pb::stream_server::Stream for EchoServer {
    type ServerStreamStream = ResponseStream;

    async fn server_stream(
        &self,
        request: tonic::Request<pb::StreamRequest>,
    ) -> Result<tonic::Response<Self::ServerStreamStream>, tonic::Status> {
        println!("The gRPC streaming....");
        println!("The client request is from {:?}", request.remote_addr());

        // creating an infinite stream...
        let repeat = repeat(pb::StreamResponse {
            message: "hello from server".to_string(),
            version: "1.0".to_string(),
        });

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_secs(1)));

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        println!("The request was queued")
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            println!("Client disconnected.....");
        });

        let output_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(output_stream) as Self::ServerStreamStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = EchoServer {};

    println!("Server is spawned and listening....");

    Server::builder()
        .add_service(pb::stream_server::StreamServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}
