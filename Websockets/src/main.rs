use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};


// so what happens here is that the handler is similar to a rest api handler 
// in rest api how we had Query, State Extractor same here we have the WebSocketUpgrade
// now what it does it simply in the request headers looks for Connection:upgrade 
// and then upgrades the connection to websocket insted of rest based and returns 101 to complete handshake
// once the connection is upgraded the handle_socket() handles the stream of data 
async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket))
}

// So here once the connection is establish and the handshake is done the socket waits 
// for a request and once received it handled it accrodingly 
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            // this is used to handle string,json etc 
            Message::Text(text) => {
                println!("Got message from client, {}", text);
                let response = format!("i got ur message {}, thank you", text);
                socket.send(Message::Text(response.into())).await.unwrap()
            }
            // during closing of a connection
            Message::Close(_) => {
                println!("Got it closing the connection");
                break;
            }
            // other types Binary (to send binary data, it is faster used in games)
            // Ping Pong (to verify if the connection still exits)
            _ => {
                println!("Not handled.....")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/send", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000",
    )
    .await
    .unwrap();

    println!("The server is running at port 3000");
    axum::serve(listener, app).await.unwrap();

}
