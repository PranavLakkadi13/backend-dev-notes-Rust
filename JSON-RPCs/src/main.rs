use jsonrpc_core::{Error, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

fn main() {
    // JSON RPC is a lightweight RPC protocol used in microservices

    let mut io = IoHandler::default();

    io.add_method("say_hello", |_param: Params| async {
        Ok(serde_json::Value::String("Hello from JSON RPC.......".to_string()))
    });

    io.add_method("add", |param: Params| async {
        let tuple = param.parse::<(u32, u32)>();

        match tuple {
            Ok((a, b)) => Ok(serde_json::Value::Number(serde_json::Number::from(a + b))),
            Err(ex) => {
                eprintln!("{:?}", ex);
                Err(Error::invalid_params("Expected 2 integers...."))
            }
        }
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"0.0.0.0:3000".parse().unwrap())
        .ok()
        .unwrap();

    println!("The server is ready....");

    server.wait();
}
