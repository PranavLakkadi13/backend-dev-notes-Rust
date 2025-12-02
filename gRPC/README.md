## gRPC (Google Remote Procedure Call)
gRPC is a modern framework for building Remote Procedure Call (RPC) systems. Think of it as a faster, more efficient version of JSON-RPC.   <br>
It uses Protocol Buffers (protobuf) for data serialization, which is more compact and faster than JSON.  <br>
gRPC supports multiple programming languages and provides features like authentication, load balancing, and bidirectional streaming.  <br>


# Proto buf 
Protocol Buffers (Protobuf) is a language-agnostic, platform-neutral serialization format developed by Google for efficiently encoding structured data.  <br>
It's used to define message schemas and services in .proto files, which are compiled into code for languages like Rust, Java, etc.  <br>
In gRPC, it handles data serialization/deserialization for RPC calls, offering better performance and smaller payloads than JSON/XML. 


# commands 
```bash
cargo run --quiet --bin grpc-client
```

```bash
cargo run --quiet --bin grpc-server
```