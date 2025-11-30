use jsonrpc_core::{Error, IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;

fn main() {
    // JSON RPC is a lightweight RPC protocol used in microservices

    let mut io = IoHandler::default();

    io.add_method("say_hello", |_param: Params| async {
        Ok(serde_json::Value::String(
            "Hello from JSON RPC.......".to_string(),
        ))
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

#[cfg(test)]
mod test {

    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    struct Addresult {
        jsonrpc: String,
        result: u64,
        id: u32,
    }

    #[derive(Serialize)]
    struct AddRequest {
        jsonrpc: String,
        method: String,
        params: Vec<u32>,
        id: u32,
    }

    #[tokio::test]
    async fn test_json_rpc_add() {
        // since json in rust can be a struct we will use a struct
        let request_body = AddRequest {
            jsonrpc: "2.0".to_string(),
            method: "add".to_string(),
            params: vec![2, 3],
            id: 1,
        };

        let response = reqwest::Client::new()
            .post("http://localhost:3000")
            .json(&request_body)
            .send()
            .await
            .unwrap();

        let body: Addresult = response.json().await.unwrap();
        assert_eq!(body.result, 5);
        assert_eq!(body.jsonrpc, "2.0".to_string());
        assert_eq!(body.id, 1);
    }
}
