// the proto folder has the schema of the rpc
// it has the methods and the type they expect the data
// proto is basically protocol buffers
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // here using tonic we will build the proto code
    tonic_build::compile_protos("proto/helloworld.proto")?;

    Ok(())
}
