# gRPC Stream Example

This project demonstrates a server-streaming gRPC service implementation in Rust using `tonic`.

## Key Takeaway

For a gRPC method to support streaming (e.g., server-side streaming), you **must** include the `stream` keyword in the `returns` definition of your Protobuf (`.proto`) file. 

### Correct Stream Definition
```protobuf
service stream {
    // Note the `stream` keyword inside `returns (...)`
    rpc ServerStream(StreamRequest) returns (stream StreamResponse) {}
}
```

If you omit the `stream` keyword (e.g., `returns (StreamResponse)`), the code generator will treat it as a unary (single request-response) RPC, causing compilation errors where the server code attempts to return or process streams.
