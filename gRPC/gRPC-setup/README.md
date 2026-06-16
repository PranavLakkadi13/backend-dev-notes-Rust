# gRPC Starter Guide: Hello World in Rust (Tonic)

Welcome to backend development and gRPC! This guide covers the basics of gRPC, Protocol Buffers, and how the code in this directory operates.

---

## What is gRPC?

**gRPC (Google Remote Procedure Call)** is a high-performance, open-source framework for communication between services. 

### Key Concepts
1. **Remote Procedure Call (RPC)**: Historically, in a REST API, you interact with resources using HTTP verbs (`GET /users`, `POST /users`). With RPC, you execute functions/methods on a remote server as if they were local functions in your own code (e.g., calling `client.say_hello(...)`).
2. **HTTP/2**: Unlike standard REST APIs which often use HTTP/1.1, gRPC runs entirely on **HTTP/2**. This allows features like:
   * **Multiplexing**: Sending multiple requests/responses over a single connection simultaneously.
   * **Streaming**: Sending streams of requests or responses (unidirectional or bidirectional).
3. **Protocol Buffers (Protobuf)**: Instead of sending heavy, text-based JSON or XML payloads, gRPC sends highly compressed binary data using Protobuf.

---

## 1. Defining the Schema: Protocol Buffers

gRPC services are strictly typed. We define messages and services in a `.proto` file (see [helloworld.proto](proto/helloworld.proto)):

```protobuf
syntax = "proto3";
package helloworld;

// The service definition.
service Greetings {
    rpc SayHello (HelloRequest) returns (HelloResponse);
}

// The request message containing the user's name.
message HelloRequest {
    string name = 1;
}

// The response message containing the greetings
message HelloResponse {
    string name = 1;
}
```

### What do the numbers (`= 1`) mean?
These are **tag numbers**. They represent the unique identifier for each field in the binary encoding of the message. Once defined, these numbers should never be changed because the serialized binary payload uses the numbers (not the names like "name") to identify fields.

---

## 2. Code Generation (`build.rs`)

Rust needs to convert the `.proto` schema file into Rust structs and traits. This is done automatically before compilation using `build.rs`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/helloworld.proto")?;
    Ok(())
}
```

When you run `cargo build`, `tonic_build` generates Rust files (e.g., defining `HelloRequest`, `GreetingsServer`, and `GreetingsClient`) which we include in our source files via `tonic::include_proto!("helloworld")`.

---

## 3. The Server (`src/server.rs`)

Let's look at [server.rs](src/server.rs):

1. **Defining the Service Handler Struct**:
   ```rust
   #[derive(Debug, Default)]
   pub struct MyGreeter {}
   ```
2. **Implementing the Service Trait**:
   We implement the `Greetings` trait (generated from the `.proto` service) on our `MyGreeter` struct:
   ```rust
   #[tonic::async_trait]
   impl Greetings for MyGreeter {
       async fn say_hello(
           &self,
           request: Request<HelloRequest>,
       ) -> Result<Response<HelloResponse>, Status> {
           let reply = HelloResponse {
               name: format!("Hello {} from server....", request.into_inner().name),
           };
           Ok(Response::new(reply))
       }
   }
   ```
   * `request.into_inner()` extracts the deserialized `HelloRequest` struct.
   * `tonic::Response::new(reply)` wraps our response payload to be sent back.
3. **Starting the Server**:
   ```rust
   Server::builder()
       .add_service(GreetingsServer::new(greeter_service))
       .serve(address)
       .await?;
   ```
   We wrap our `MyGreeter` inside `GreetingsServer` (which handles the gRPC protocol/network translation) and serve it on port `50051`.

---

## 4. The Client (`src/client.rs`)

Let's look at [client.rs](src/client.rs):

1. **Connecting**:
   ```rust
   let mut client = GreetingsClient::connect("http://[::1]:50051").await?;
   ```
   We create a connection channel to the gRPC server.
2. **Sending the Request**:
   ```rust
   let request = tonic::Request::new(HelloRequest {
       name: "Alice".to_string(),
   });
   let response = client.say_hello(request).await?;
   ```
   We instantiate the generated `HelloRequest` and call `.say_hello` on the client stub just like calling a local method.

---

## Running the Application

Open two terminal windows:

### Terminal 1: Run the Server
```bash
cargo run --bin grpc-server
```

### Terminal 2: Run the Client
```bash
cargo run --bin grpc-client
```