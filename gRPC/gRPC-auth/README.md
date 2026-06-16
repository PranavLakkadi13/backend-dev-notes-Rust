# gRPC Authentication with Interceptors in Tonic

This project demonstrates how to implement authentication in a gRPC client-server application using interceptors in Rust with the `tonic` library.

## Goal

The objective is to append a secure authentication token (specifically a `Bearer` token) to the metadata of every outgoing gRPC request from the client, and validate that token on the server before allowing the request to reach the service handler.

---

## What is an Interceptor?

An **interceptor** acts as middleware for gRPC requests and responses. It intercepts outgoing requests on the client side, or incoming requests on the server side, allowing you to modify them (e.g., adding metadata/headers) or inspect/validate them (e.g., authentication checks) before they reach their final destination.

---

## 1. Client-Side Interceptor

In [client.rs](src/client.rs), the client is initialized with an interceptor:

```rust
let mut client = GreetingsClient::with_interceptor(channel, move |mut req: Request<()>| {
    req.metadata_mut().insert("authorization", token.clone());
    Ok(req)
});
```

* **What it receives:** The closure accepts a `Request<()>` representing the outgoing request. The payload type is `()` because the interceptor only deals with request metadata (headers) and transport configuration—not the actual protobuf payload (e.g., `HelloRequest`) which hasn't been serialized yet.
* **What it does:**
  1. It grabs a mutable reference to the request's metadata map using `req.metadata_mut()`.
  2. It inserts the authorization token under the `"authorization"` key.
  3. It returns `Ok(req)` to allow the request to proceed.
* **Why it uses `move`:** The `move` keyword transfers ownership of the `token` variable into the closure since the closure will be called on every subsequent gRPC request.

---

## 2. Server-Side Interceptor

In [server.rs](src/server.rs), incoming requests are intercepted and validated using a dedicated function:

```rust
fn authenticate(req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(token) if token.to_str().unwrap().starts_with("Bearer") => Ok(req),
        _ => Err(Status::unauthenticated("invalid auth")),
    }
}
```

* **What it receives:** Like the client side, it takes a `Request<()>` containing the incoming request metadata.
* **What it does:**
  1. It inspects the metadata to find the `"authorization"` header.
  2. If the token is found and starts with `"Bearer"`, it returns `Ok(req)`. The request is allowed through, and tonic will proceed to deserialize the payload and invoke your service handler (`say_hello`).
  3. If the token is missing or malformed, it returns `Err(Status::unauthenticated("invalid auth"))`. The request is rejected immediately, and the service handler is never executed.
